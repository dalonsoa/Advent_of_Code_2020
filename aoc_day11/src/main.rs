use ndarray::{s, Array2};
use std::env;
use std::fs;

/// Read file into vector of strings
fn read_file(args: &[String]) -> Vec<Vec<char>> {
    let filename = &args[1];
    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split("\n")
        .map(|s| s.to_string().chars().collect())
        .collect()
}

/// Converts the 2D input characters to a numeric representation and add 0 all around
///
/// . -> 0
/// L -> 1
/// # -> 10
fn to_array(raw: &Vec<Vec<char>>) -> Array2<usize> {
    let mut result = Array2::<usize>::zeros((raw.len() + 2, raw[0].len() + 2));

    for (i, row) in raw.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                '.' => result[[i + 1, j + 1]] = 0,
                'L' => result[[i + 1, j + 1]] = 1,
                _ => result[[i + 1, j + 1]] = 10,
            }
        }
    }
    return result;
}

/// Updates a seat possition based on the occupation of the seats around
fn update_seat_old(seats: &Array2<usize>, r: usize, c: usize) -> usize {
    if seats[[r, c]] == 0 {
        return 0;
    }
    let sum = &seats.slice(s![r - 1..r + 2, c - 1..c + 2]).to_owned().sum() - seats[[r, c]];
    match sum {
        0..=9 => return 10,
        10..=39 => return seats[[r, c]],
        _ => return 1,
    }
}

/// New version of the update of a seat possition based on the occupation of the seats around
fn update_seat_new(seats: &Array2<usize>, r: usize, c: usize) -> usize {
    if seats[[r, c]] == 0 {
        return 0;
    }
    let dir = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut sum = 0;
    for d in dir.iter() {
        sum += find_seat(seats, r, c, d)
    }

    match sum {
        0..=9 => return 10,
        10..=49 => return seats[[r, c]],
        _ => return 1,
    }
}

/// Finds value of the first seat in the given direction
fn find_seat(seats: &Array2<usize>, r: usize, c: usize, d: &(isize, isize)) -> usize {
    let nr = (r as isize + d.0) as usize;
    let nc = (c as isize + d.1) as usize;
    match seats.get((nr, nc)) {
        Some(s) => {
            if *s == 0 {
                return find_seat(seats, nr, nc, d);
            } else {
                return *s;
            }
        }
        None => return 0,
    }
}

/// Update seat plan once by updating each seat sequentially
fn update_seat_plan_once(
    seats: &Array2<usize>,
    method: fn(&Array2<usize>, usize, usize) -> usize,
) -> Array2<usize> {
    let mut result = seats.clone();

    for ((r, c), _) in seats.indexed_iter() {
        result[[r, c]] = method(&seats, r, c)
    }
    return result;
}

/// Update seat plan until convergence is reached or a maximum number of iterations
fn update_seat_plan(
    seats: &Array2<usize>,
    maxiter: usize,
    method: fn(&Array2<usize>, usize, usize) -> usize,
) -> Array2<usize> {
    if maxiter == 0 {
        panic!("Too many iterations!");
    }

    let result = update_seat_plan_once(seats, method);
    if result == *seats {
        return result;
    } else {
        return update_seat_plan(&result, maxiter - 1, method);
    }
}

/// Count occupied seats
fn count_occupied(seats: &Array2<usize>) -> usize {
    seats.mapv(|a| (a == 10) as usize).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let initial = to_array(&read_file(&args));

    // Count the number of occupied seats after convergence
    let filled = &update_seat_plan(&initial, 1000, update_seat_old);
    println!(
        "The number of occupied seats is {}",
        count_occupied(&filled)
    );

    // Count the number of occupied seats using the new method
    let filled = &update_seat_plan(&initial, 1000, update_seat_new);
    println!(
        "The number of occupied seats using the new method {}",
        count_occupied(&filled)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    const SEATS: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const SEATS_1: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    const SEATS_2: &str = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    const FINAL: &str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    const FINAL_2: &str = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    #[test]
    fn test_matrix_to_numeric() {
        let raw = vec![vec!['.', '#', 'L'], vec!['.', '#', 'L']];
        let expected = arr2(&[
            [0, 0, 0, 0, 0],
            [0, 0, 10, 1, 0],
            [0, 0, 10, 1, 0],
            [0, 0, 0, 0, 0],
        ]);
        assert_eq!(expected, to_array(&raw))
    }

    #[test]
    fn test_update_seat_old() {
        let seats = arr2(&[[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
        assert_eq!(update_seat_old(&seats, 1, 1), 0);

        let seats = arr2(&[[0, 0, 0], [0, 1, 0], [0, 0, 0]]);
        assert_eq!(update_seat_old(&seats, 1, 1), 10);

        let seats = arr2(&[[0, 0, 10], [10, 10, 0], [10, 0, 0]]);
        assert_eq!(update_seat_old(&seats, 1, 1), 10);

        let seats = arr2(&[[0, 0, 10], [10, 10, 0], [10, 0, 10]]);
        assert_eq!(update_seat_old(&seats, 1, 1), 1);
    }

    #[test]
    fn test_update_seat_plan_once() {
        let initial = to_array(
            &SEATS
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        let expected1 = to_array(
            &SEATS_1
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(expected1, update_seat_plan_once(&initial, update_seat_old));

        let expected2 = to_array(
            &SEATS_2
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(
            expected2,
            update_seat_plan_once(&expected1, update_seat_old)
        )
    }

    #[test]
    fn test_update_seat_plan() {
        let initial = to_array(
            &SEATS
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        let expected = to_array(
            &FINAL
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(expected, update_seat_plan(&initial, 10, update_seat_old))
    }

    #[test]
    fn test_count_ocupied() {
        let initial = to_array(
            &SEATS
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(
            37,
            count_occupied(&update_seat_plan(&initial, 10, update_seat_old))
        )
    }

    #[test]
    fn test_update_seat_plan_new() {
        let initial = to_array(
            &SEATS
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        let expected = to_array(
            &FINAL_2
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(expected, update_seat_plan(&initial, 10, update_seat_new))
    }

    #[test]
    fn test_count_ocupied_new() {
        let initial = to_array(
            &SEATS
                .split("\n")
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        );
        assert_eq!(
            26,
            count_occupied(&update_seat_plan(&initial, 10, update_seat_new))
        )
    }
}
