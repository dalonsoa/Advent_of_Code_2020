use regex::Regex;
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
    records.iter().filter(|s| is_valid(&s)).count()
}

/// Count the number of trully valid records based on the keys they have
fn count_trully_valid(records: &Vec<HashMap<String, String>>) -> usize {
    records.iter().filter(|s| is_trully_valid(&s)).count()
}

/// Cheks if a particular record is valid... in principle
fn is_valid(record: &HashMap<String, String>) -> bool {
    record.len() == 8 || (record.len() == 7 && !record.contains_key("cid"))
}

/// Checks if a particular record is trully valid
fn is_trully_valid(record: &HashMap<String, String>) -> bool {
    if is_valid(&record) {
        return validate_byr(&record["byr"])
            && validate_iyr(&record["iyr"])
            && validate_eyr(&record["eyr"])
            && validate_hgt(&record["hgt"])
            && validate_ecl(&record["ecl"])
            && validate_hcl(&record["hcl"])
            && validate_pid(&record["pid"]);
    }
    return false;
}

fn validate_byr(byr: &str) -> bool {
    let date = byr.parse().unwrap();
    1920 <= date && date <= 2002
}

fn validate_iyr(iyr: &str) -> bool {
    let date = iyr.parse().unwrap();
    2010 <= date && date <= 2020
}

fn validate_eyr(eyr: &str) -> bool {
    let date = eyr.parse().unwrap();
    2020 <= date && date <= 2030
}

fn validate_hgt(hgt: &str) -> bool {
    let cm = Regex::new(r"^(\d{3})cm$").unwrap();
    let inches = Regex::new(r"^(\d{2})in$").unwrap();

    if hgt.contains("cm") {
        let value = cm.captures(hgt);
        match value {
            None => return false,
            _ => {
                let v = value.unwrap()[1].parse().unwrap();
                return 150 <= v && v <= 193;
            }
        }
    } else if hgt.contains("in") {
        let value = inches.captures(hgt);
        match value {
            None => return false,
            _ => {
                let v = value.unwrap()[1].parse().unwrap();
                return 59 <= v && v <= 76;
            }
        }
    }
    return false;
}

fn validate_hcl(hcl: &str) -> bool {
    let re = Regex::new(r"^(#[0-9a-f]{6})$").unwrap();
    match re.captures(hcl) {
        None => return false,
        _ => return true,
    }
}

fn validate_ecl(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn validate_pid(pid: &str) -> bool {
    let re = Regex::new(r"^(\d{9})$").unwrap();
    match re.captures(pid) {
        None => return false,
        _ => return true,
    }
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

    // Number of trully valid passports
    let valid = count_trully_valid(&passports);
    println!("Trully valid passports: {}", valid);
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

    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

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

    #[test]
    fn test_trully_valid() {
        let invalid = split_records(split_batch(INVALID));
        let valid = split_records(split_batch(VALID));
        assert_eq!(count_trully_valid(&invalid), 0);
        assert_eq!(count_trully_valid(&valid), 4);
    }

    #[test]
    fn test_validate_byr() {
        assert!(validate_byr("1930"));
        assert!(!validate_byr("930"));
    }

    #[test]
    fn test_validate_iyr() {
        assert!(validate_iyr("2015"));
        assert!(!validate_iyr("930"));
    }

    #[test]
    fn test_validate_eyr() {
        assert!(validate_eyr("2025"));
        assert!(!validate_eyr("2930"));
    }

    #[test]
    fn test_validate_hgt() {
        assert!(!validate_hgt("60"));
        assert!(validate_hgt("160cm"));
        assert!(!validate_hgt("60cm"));
        assert!(validate_hgt("70in"));
        assert!(!validate_hgt("200in"));
    }

    #[test]
    fn test_validate_hcl() {
        assert!(validate_hcl("#124af4"));
        assert!(!validate_hcl("124af4"));
        assert!(!validate_hcl("#124f4"));
        assert!(!validate_hcl("#1m4f44"));
    }

    #[test]
    fn test_validate_ecl() {
        assert!(validate_ecl("blu"));
        assert!(!validate_ecl("foo"));
        assert!(!validate_ecl("blufoo"));
    }

    #[test]
    fn test_validate_pid() {
        assert!(validate_pid("000123442"));
        assert!(!validate_pid("00123442"));
        assert!(!validate_pid("0001234424"));
        assert!(!validate_pid("0001n3442"));
        assert!(!validate_pid("000123442n"));
    }
}
