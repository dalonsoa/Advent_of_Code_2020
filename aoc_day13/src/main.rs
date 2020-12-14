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

/// Reads the arrival timestamp and the bus list
fn parse_inputs(raw: &Vec<String>) -> (isize, Vec<isize>) {
    let arrival = raw[0].parse::<isize>().unwrap();
    let buses = raw[1]
        .split(",")
        .map(|s| match s.parse::<isize>() {
            Ok(num) => num,
            Err(_) => -1,
        })
        .collect();
    return (arrival, buses);
}

/// Finds outs what is the next bus and how much longer it will take to arrive
fn next_bus(arrival: isize, buses: Vec<isize>) -> (isize, isize) {
    let mut times = buses
        .iter()
        .filter(|&&b| b > 0)
        .map(|&b| (b, b - arrival % b))
        .collect::<Vec<(isize, isize)>>();
    times.sort_by(|a, b| a.1.cmp(&b.1));
    return times[0];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (arrival, buses) = parse_inputs(&read_file(&args));

    // The next bus arrival information is
    let bus = next_bus(arrival, buses);
    println!(
        "The next bus number multiplied by the waiting time is {}",
        bus.0 * bus.1
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_parse_inputs() {
        let inputs = INPUT
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (arrival, buses) = parse_inputs(&inputs);
        assert_eq!(arrival, 939);
        assert_eq!(buses, [7, 13, -1, -1, 59, -1, 31, 19])
    }

    #[test]
    fn test_next_bus() {
        let arrival: isize = 939;
        let buses: Vec<isize> = vec![7, 13, -1, -1, 59, -1, 31, 19];
        assert_eq!(next_bus(arrival, buses), (59, 5))
    }

    #[test]
    fn test_count_configs() {}
}
