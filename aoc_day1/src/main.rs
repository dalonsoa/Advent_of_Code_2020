use std::env;
use std::fs;

/// Reads a text file with integer numbers in a column
fn get_nums(args: &[String]) -> Vec<i32> {
    let filename = &args[1];

    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Find two numbers that add up to a third one, the target
/// 
/// Recursive function that scans a vector and looks for two numbers that 
/// add up to a third one. If found, return their product. Otherwise, return zero.
fn sum_two_nums(num1: &i32, numbers: &[i32], target: &i32) -> i32 {
    
    for num2 in numbers.iter() {
        if num1 + num2 == *target {
            return num1 * num2;
        }
    };

    if numbers.len() <= 2 {
        return 0
    } else {
        sum_two_nums(&numbers[0], &numbers[1..], &target)
    }
}

/// Find three numbers that add up to a third one, the target
/// 
/// Recursive function that scans a vector and looks for three numbers that 
/// add up to a third one. If found, return their product. Otherwise, return zero.
fn sum_three_nums(num1: &i32, numbers: &[i32], target: &i32) -> i32 {
    let mut counter: usize = 1;

    for num2 in numbers.iter() {
        if num1 + num2 < *target {
            for num3 in numbers[counter..].iter() {
                if num1 + num2 + num3 == *target {
                    return num1 * num2 * num3;
                }
            }
        }
        counter += 1
    };

    if numbers.len() <= 3 {
        return 0
    } else {
        sum_three_nums(&numbers[0], &numbers[1..], &target)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers: Vec<i32> = get_nums(&args);

    println!("Two numbers: {:?}", sum_two_nums(&numbers[0], &numbers[1..], &2020));
    println!("Three numbers: {:?}", sum_three_nums(&numbers[0], &numbers[1..], &2020));
}
