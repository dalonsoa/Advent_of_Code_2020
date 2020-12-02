use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Password {
    low: usize,
    high: usize,
    letter: char,
    pwd: Vec<char>,
}

impl Password {
    /// Parse a string using Regex to get the password parameters
    ///
    /// The input string must have format "low-high letter: password"
    ///
    /// It is not fast...
    fn from_str(raw: String) -> Password {
        let re = Regex::new(r"(\d+)\W(\d+)\s(\w)\W+(\w+)").unwrap();
        let data = re.captures(&raw).unwrap();

        Password {
            low: data.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            high: data.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: data.get(3).unwrap().as_str().parse::<char>().unwrap(),
            pwd: data.get(4).unwrap().as_str().chars().collect(),
        }
    }

    /// Validates the password according to the old policy
    fn is_valid_old(&self) -> bool {
        let c: usize = self.pwd.iter().filter(|s| **s == self.letter).count();
        c >= self.low && c <= self.high
    }

    /// Validates the password according to the new policy
    fn is_valid_new(&self) -> bool {
        (self.pwd[self.low - 1] == self.letter) ^ (self.pwd[self.high - 1] == self.letter)
    }
}

/// Reads a text file with policy and passwords
fn get_passwords(args: &[String]) -> Vec<Password> {
    let filename = &args[1];

    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split("\n")
        .map(|s| Password::from_str(s.to_string()))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let passwords: Vec<Password> = get_passwords(&args);
    let mut valid_old: usize = 0;
    let mut valid_new: usize = 0;

    // Checking old policy
    println!("Checking old policy...");
    for pwd in passwords.iter() {
        valid_old += pwd.is_valid_old() as usize
    }
    println!(
        "Old policy valid passwords: {}/{}",
        valid_old,
        passwords.len()
    );

    // Checking new policy
    println!("Checking new policy...");
    for pwd in passwords.iter() {
        valid_new += pwd.is_valid_new() as usize
    }
    println!(
        "New policy valid passwords: {}/{}",
        valid_new,
        passwords.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_policy() {
        let mut valid: usize = 0;
        let passwords: Vec<&str> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        for pwd in passwords.iter() {
            valid += Password::from_str(pwd.to_string()).is_valid_old() as usize
        }
        assert_eq!(valid, 2)
    }

    #[test]
    fn test_new_policy() {
        let mut valid: usize = 0;
        let passwords: Vec<&str> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        for pwd in passwords.iter() {
            valid += Password::from_str(pwd.to_string()).is_valid_new() as usize
        }
        assert_eq!(valid, 1)
    }
}
