use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Password {
    low: usize,
    high: usize,
    letter: String,
    pwd: String,
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
            letter: data.get(3).unwrap().as_str().to_string(),
            pwd: data.get(4).unwrap().as_str().to_string(),
        }
    }

    /// Validates the password according to the old policy
    fn is_valid_old(&self) -> bool {
        let c = self.pwd.matches(&self.letter).count();
        c >= self.low && c <= self.high
    }

    /// Validates the password according to the new policy
    fn is_valid_new(&self) -> bool {
        (self.pwd.as_bytes()[self.low - 1] == self.letter.as_bytes()[0])
            ^ (self.pwd.as_bytes()[self.high - 1] == self.letter.as_bytes()[0])
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
