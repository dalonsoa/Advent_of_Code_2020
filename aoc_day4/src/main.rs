use std::collections::HashMap;
use std::env;
use std::fs;

/// Split batch file into individual records
fn split_batch(batch: &str) -> Vec<String> {
    batch.split("\n\n").map(|s| s.to_string()).collect()
}

/// Split the records into the individual key:value pairs
///
/// First, it splits each record into groups of key:value at each space and new line.
/// Then, this is further split into the : and the result stored into a HashMap
fn split_records(records: Vec<String>) -> Vec<HashMap<String, String>> {
    records
        .iter()
        .map(|s| {
            s.split(|c| c == '\n' || c == ' ')
                .map(|s| s.split(":"))
                .map(|mut s| (s.next().unwrap().into(), s.next().unwrap().into()))
                .collect::<HashMap<String, String>>()
        })
        .collect()
}

/// Count the number of valid records based on the keys they have
fn count_valid(records: &Vec<HashMap<String, String>>) -> usize {
    records
        .iter()
        .filter(|s| s.len() == 8 || (s.len() == 7 && !s.contains_key("cid")))
        .count()
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
    let passports: Vec<HashMap<String, String>> = split_records(split_batch(&read_file(&args)));

    // Number of valid passports
    let valid = count_valid(&passports);
    println!("Valid passports: {}", valid);
}

#[cfg(test)]
mod tests {

    use super::*;

    const BATCH: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_split_batch() {
        assert_eq!(split_batch(BATCH).len(), 4);
    }

    #[test]
    fn test_split_records() {
        let split = split_records(split_batch(BATCH));
        println!("{:?}", split[0]);
        assert_eq!(split[0].len(), 8);
        assert_eq!(split[1].len(), 7);
        assert_eq!(split[2].len(), 7);
        assert_eq!(split[3].len(), 6);
    }

    #[test]
    fn test_valid() {
        let split = split_records(split_batch(BATCH));
        assert_eq!(count_valid(&split), 2);
    }
}
