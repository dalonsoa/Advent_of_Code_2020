use std::env;
use std::fs;

/// Convert the input string into a vecotr of vecotor of characters
fn string_to_array(data: &str) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|s| s.to_string().chars().collect())
        .collect()
}

/// Count the trees at the given positins
fn count_trees(forest: &Vec<Vec<char>>, col: usize, row: usize, tchar: char) -> usize {
    let mut trees: usize = 0;
    let mut done: usize = 0;
    let col_max: usize = forest[0].len();
    for line in forest.iter().step_by(row) {
        trees += (line[(col * done).rem_euclid(col_max)] == tchar) as usize;
        done += 1;
    }
    return trees;
}

/// Multiply trees encountered when trying multiple slopes
fn multiply_trees(forest: &Vec<Vec<char>>, slopes: Vec<(usize, usize)>, tchar: char) -> usize {
    let mut total_trees = 1;
    for slope in slopes.iter() {
        total_trees *= count_trees(forest, slope.0, slope.1, tchar);
    }
    return total_trees;
}

/// Read file into a string
fn read_file(args: &[String]) -> String {
    let filename = &args[1];
    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let forest: Vec<Vec<char>> = string_to_array(&read_file(&args));

    // Number of trees at the slope locations
    let trees = count_trees(&forest, 3, 1, 0x23.into());
    println!("Trees encountered: {}", trees);

    // Total trees encounter in all slopes
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let total_trees = multiply_trees(&forest, slopes, 0x23.into());
    println!("Trees encountered multiplied: {}", total_trees);
}

#[cfg(test)]
mod tests {

    use super::*;

    const FOREST: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_string_to_array() {
        let result = string_to_array(FOREST);
        assert_eq!(result.len(), 11);
        assert_eq!(result[0].len(), 11);
    }

    #[test]
    fn test_count_trees() {
        let forest = string_to_array(FOREST);
        let trees = count_trees(&forest, 3, 1, 0x23.into());
        assert_eq!(trees, 7);
    }

    #[test]
    fn test_try_multiple_slopes() {
        let forest = string_to_array(FOREST);
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let total_trees = multiply_trees(&forest, slopes, 0x23.into());
        assert_eq!(total_trees, 336);
    }
}
