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

/// Sort existing adapters and adds the socket and the built in one
fn sort_adapters(adapt: &Vec<usize>) -> Vec<usize> {
    let mut sorted = adapt.clone();
    sorted.sort();
    sorted.insert(0, 0);
    sorted.push(sorted.iter().max().unwrap() + 3);
    return sorted;
}

/// Get the difference in jolts between the adapters and count them
///
/// The input list of adapters muts be sorted
fn count_diffs(adapt: &Vec<usize>) -> Vec<usize> {
    let mut diffs: Vec<usize> = vec![0, 0, 0];
    for (i, m) in adapt[1..].iter().enumerate() {
        diffs[m - adapt[i] - 1] += 1;
    }
    return diffs;
}

/// Counts all the possible configurations of the given subset of adapters
fn _count_configs(adapt: &[usize], lowest: &usize) -> usize {
    let mut count = 1;
    for i in *lowest..adapt.len() - 1 {
        if adapt[i + 1] - adapt[i - 1] <= 3 {
            count += _count_configs(&[&adapt[..i], &adapt[i + 1..]].concat(), &i)
        }
    }
    return count;
}

/// Counts all the possible configurations of arranging the adapters
///
/// The configurations of the groups with adapters separated by less than 3 jolts are
/// first counted and the results multiplied with each other.
fn count_configs(adapt: &[usize]) -> usize {
    let mut count = 1;
    let mut group: Vec<usize> = vec![adapt[0]];

    for m in adapt[1..].iter() {
        if m - group.last().unwrap() == 3 {
            if group.len() > 2 {
                count *= _count_configs(&group[..], &1);
            }
            group.clear();
        }
        group.push(*m);
    }

    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let adapt = sort_adapters(&read_file(&args));

    // Find the number of 1-jolt differences multiplied by the number of
    // 3-jolt differences
    let diffs = count_diffs(&adapt);
    println!("The product is {}", diffs[0] * diffs[2]);

    // Find the number of all disctinct configurations adapters can be connected
    println!("The number of configurations is {}", count_configs(&adapt))
}

#[cfg(test)]
mod tests {

    use super::*;

    const ADAPT_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const ADAPT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_sort_adapters() {
        let adapt: Vec<usize> = ADAPT_1.split("\n").map(|s| s.parse().unwrap()).collect();
        let expected = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        assert_eq!(expected, sort_adapters(&adapt))
    }

    #[test]
    fn test_count_diffs() {
        let adapt: Vec<usize> = ADAPT_1.split("\n").map(|s| s.parse().unwrap()).collect();
        let expected = vec![7, 0, 5];
        let actual = count_diffs(&sort_adapters(&adapt));
        assert_eq!(expected, actual);

        let adapt: Vec<usize> = ADAPT_2.split("\n").map(|s| s.parse().unwrap()).collect();
        let expected = vec![22, 0, 10];
        let actual = count_diffs(&sort_adapters(&adapt));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_configs() {
        let adapt: Vec<usize> = ADAPT_1.split("\n").map(|s| s.parse().unwrap()).collect();
        assert_eq!(count_configs(&sort_adapters(&adapt)[..]), 8);

        let adapt: Vec<usize> = ADAPT_2.split("\n").map(|s| s.parse().unwrap()).collect();
        assert_eq!(count_configs(&sort_adapters(&adapt)[..]), 19208);
    }
}
