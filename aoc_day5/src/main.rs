use regex::Regex;
use std::env;
use std::fs;

/// Read file into a string
fn read_file(args: &[String]) -> String {
    let filename = &args[1];
    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .to_string()
}

/// Transform the input sequence of letters into a string of 0s and 1s
fn to_binary_str(raw: &str) -> Vec<String> {
    let ones = Regex::new(r"[BR]").unwrap();
    let zeros = Regex::new(r"[FL]").unwrap();

    ones.replace_all(&zeros.replace_all(raw, "0"), "1")
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn factory(binary_str: &str) -> Seat {
        let row = &binary_str[..7];
        let col = &binary_str[7..];
        Seat {
            row: usize::from_str_radix(row, 2).unwrap(),
            col: usize::from_str_radix(col, 2).unwrap(),
        }
    }

    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

/// Finds your seat id
///
/// The condition is that it should not be in the id list but the +1 and -1 should.
fn find_id(ids: &Vec<usize>) -> usize {
    let maxi = *ids.iter().max().unwrap();
    let mini = *ids.iter().min().unwrap();

    for id in mini..maxi {
        if !ids.contains(&id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)) {
            return id;
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary_str = to_binary_str(&read_file(&args));
    let seats: Vec<Seat> = binary_str.iter().map(|s| Seat::factory(s)).collect();
    let ids = seats.iter().map(|s| s.id()).collect::<Vec<usize>>();

    // Maximum id
    let maxi = ids.iter().max();
    println!("Maximum seat id is {:?}", maxi.unwrap());

    // Your id is...
    let your_id = find_id(&ids);
    match your_id {
        0 => println!("Your id was not found!!"),
        _ => println!("Your id is {}", your_id),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SEATS: &str = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_to_binary_str() {
        let binary_str = to_binary_str(SEATS);
        assert_eq!(binary_str[0], "0101100101");
        assert_eq!(binary_str[1], "1000110111");
        assert_eq!(binary_str[2], "0001110111");
        assert_eq!(binary_str[3], "1100110100");
    }

    #[test]
    fn test_seat_factory() {
        let binary_str = to_binary_str(SEATS);
        let expected = [(44, 5), (70, 7), (14, 7), (102, 4)];

        for (s, exp) in binary_str.iter().zip(expected.iter()) {
            let seat = Seat::factory(s);
            assert_eq!(seat.row, exp.0);
            assert_eq!(seat.col, exp.1);
        }
    }

    #[test]
    fn test_seat_id() {
        let binary_str = to_binary_str(SEATS);
        let expected = [357, 567, 119, 820];

        for (s, exp) in binary_str.iter().zip(expected.iter()) {
            let seat = Seat::factory(s);
            assert_eq!(seat.id(), *exp);
        }
    }

    #[test]
    fn test_largest_id() {
        let binary_str = to_binary_str(SEATS);
        let seats: Vec<usize> = binary_str.iter().map(|s| Seat::factory(s).id()).collect();
        assert_eq!(seats.iter().max(), Some(&820))
    }
}
