use std::env;
use std::fs;

/// Read file into vector of strings
fn read_file(args: &[String]) -> Vec<usize> {
    let filename = &args[1];
    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Checks if a particular number is valid
///
/// It scans the previous X numbers in search of two that can result in that value.
fn check_valid(code: &Vec<usize>, w: &usize, i: &usize, num: &usize) -> bool {
    for n in code[i - w..*i].iter() {
        for m in code[i - w..*i].iter() {
            if n != m && n + m == *num {
                return true;
            }
        }
    }
    return false;
}

/// Finds the first invalid value of the code
fn find_first_invalid(code: &Vec<usize>, w: &usize) -> Option<usize> {
    for (i, num) in code.iter().enumerate() {
        if i < *w {
            continue;
        }
        if !check_valid(&code, &w, &i, &num) {
            return Some(*num);
        }
    }
    return None;
}

/// Finds the code weakness
///
/// This is done by first finding the contiguos set of at least two numbers that add
/// up to the invalid one, and then adding together the first and last of the sequence.
fn find_weakness(code: &Vec<usize>, invalid: &usize) -> Option<usize> {
    for (i, num) in code.iter().enumerate() {
        let mut acc = vec![*num];
        let mut counter = i + 1;
        while acc.iter().sum::<usize>() < *invalid {
            acc.push(code[counter]);
            if acc.iter().sum::<usize>() == *invalid {
                return Some(acc.iter().min().unwrap() + acc.iter().max().unwrap());
            }
            counter += 1;
        }
    }
    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = read_file(&args);

    // Scans the code looking for the first invalid value
    let invalid = find_first_invalid(&code, &25);
    match invalid {
        Some(num) => println!("The first invalid value is {}", num),
        None => println!("No invalid value was found!!"),
    };

    // Use the invalid value to find the code weakness
    match find_weakness(&code, &invalid.unwrap()) {
        Some(num) => println!("The code weakness is {}", num),
        None => println!("No weakness was found!!"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const CODE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_check_valid() {
        let code: Vec<usize> = CODE.split("\n").map(|s| s.parse().unwrap()).collect();
        let w = 5;
        for (i, num) in code.iter().enumerate() {
            if i < w {
                continue;
            }
            if *num == 127 {
                assert!(!check_valid(&code, &w, &i, &num));
            } else {
                assert!(check_valid(&code, &w, &i, &num));
            }
        }
    }

    #[test]
    fn test_find_first_invalid() {
        let code: Vec<usize> = CODE.split("\n").map(|s| s.parse().unwrap()).collect();
        match find_first_invalid(&code, &5) {
            Some(num) => assert_eq!(num, 127),
            None => assert!(false),
        }
    }

    #[test]
    fn test_find_weakness() {
        let code: Vec<usize> = CODE.split("\n").map(|s| s.parse().unwrap()).collect();
        let invalid = 127;
        match find_weakness(&code, &invalid) {
            Some(num) => assert_eq!(num, 62),
            None => assert!(false),
        }
    }
}
