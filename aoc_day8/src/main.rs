use std::env;
use std::fs;

/// Read file into vector of strings
fn read_file(args: &[String]) -> Vec<String> {
    let filename = &args[1];
    println!("Reading {}", &args[1]);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

/// Parses a single instruction of code
///
/// The output is a tuple with the next step and the value to accumulate
fn parse_order(order: &str) -> (isize, isize) {
    let mut o = order.split_ascii_whitespace();

    match o.next().unwrap() {
        "acc" => return (1, o.next().unwrap().parse::<isize>().unwrap()),
        "jmp" => return (o.next().unwrap().parse::<isize>().unwrap(), 0),
        _ => return (1, 0),
    }
}

/// Executes a full sequence of orders
///
/// The output is the last order executed and the accumualtor at that time
fn execute_code(
    current: usize,
    code: &Vec<String>,
    acc: isize,
    mut done: Vec<usize>,
) -> (isize, isize) {
    let (step, val) = parse_order(&code[current]);
    done.push(current);
    if done.contains(&((current as isize + step) as usize)) {
        return (current as isize, acc + val);
    } else if (current as isize + step) as usize == code.len() {
        return (-1, acc + val);
    } else {
        return execute_code((current as isize + step) as usize, code, acc + val, done);
    }
}

/// Repairs the code by changing sequentially jmp and nop with each other
///
/// After changing that, it runs the code and checks if the current output is -1
fn repair_code(code: &Vec<String>) -> (isize, isize) {
    for (i, order) in code.iter().enumerate() {
        let mut new_code = code.clone();
        if order.contains("jmp") {
            new_code[i] = order.replace("jmp", "nop");
        } else if order.contains("nop") {
            new_code[i] = order.replace("nop", "jmp");
        }
        let (status, acc) = execute_code(0, &new_code, 0, Vec::new());
        if status == -1 {
            return (status, acc);
        }
    }
    return (-2, 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = read_file(&args);

    // Value in accumulator before infinite loop
    let (_, accumulator) = execute_code(0, &code, 0, Vec::new());
    println!("The accumulator before infinite loop was {}", accumulator);

    // Solve the problem and get the accumulator
    let (_, accumulator) = repair_code(&code);
    println!("The accumulator after rapairing code is {}", accumulator);
}

#[cfg(test)]
mod tests {

    use super::*;

    const CODE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse_order() {
        let code: Vec<String> = CODE.split("\n").map(|s| s.to_string()).collect();
        let expected = [
            (1, 0),
            (1, 1),
            (4, 0),
            (1, 3),
            (-3, 0),
            (1, -99),
            (1, 1),
            (-4, 0),
            (1, 6),
        ];

        for (c, exp) in code.iter().zip(expected.iter()) {
            let actual = parse_order(c);
            assert_eq!(actual.0, exp.0);
            assert_eq!(actual.1, exp.1);
        }
    }

    #[test]
    fn test_execute_code() {
        let code: Vec<String> = CODE.split("\n").map(|s| s.to_string()).collect();
        let (current, accumulator) = execute_code(0, &code, 0, Vec::new());
        assert_eq!(current, 4);
        assert_eq!(accumulator, 5)
    }

    #[test]
    fn test_repair_code() {
        let code: Vec<String> = CODE.split("\n").map(|s| s.to_string()).collect();
        let (current, accumulator) = repair_code(&code);
        assert_eq!(current, -1);
        assert_eq!(accumulator, 8)
    }
}
