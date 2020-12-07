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

/// Split input file into individual groups
fn split_groups(batch: &str) -> Vec<String> {
    batch.split("\n\n").map(|s| s.to_string()).collect()
}

/// Split grpups into individual passengers
fn split_passengers(groups: &Vec<String>) -> Vec<Vec<String>> {
    groups
        .iter()
        .map(|s| s.split("\n").map(|s| s.to_string()).collect())
        .collect()
}

/// Count number of unique YES answers in group
fn count_unique_in_group(groups: &Vec<Vec<String>>) -> Vec<usize> {
    let mut yes_answers = Vec::new();

    for g in groups.iter() {
        let mut pass_answers: Vec<char> = g.join("").chars().collect();
        pass_answers.sort_by(|a, b| a.cmp(b));
        pass_answers.dedup();
        yes_answers.push(pass_answers.len())
    }
    return yes_answers;
}

/// Count number of unique YES answers in group
fn count_common_in_group(groups: &Vec<Vec<String>>) -> Vec<usize> {
    let mut common_answers = Vec::new();

    for g in groups.iter() {
        if g.len() == 1 {
            common_answers.push(g[0].len())
        } else {
            let pass_answers: Vec<char> = g[0]
                .chars()
                .filter(|&c| g.iter().all(|s| s.contains(c)))
                .collect();
            common_answers.push(pass_answers.len());
        }
    }
    return common_answers;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let split = split_passengers(&split_groups(&read_file(&args)));

    // Total number of questions someone answered YES
    let yes_answers = count_unique_in_group(&split);
    println!(
        "Questions answered Yes - {:?}",
        yes_answers.iter().sum::<usize>()
    );

    // Total number of questions all answered YES
    let common_answers = count_common_in_group(&split);
    println!(
        "Questions answered Yes by all - {:?}",
        common_answers.iter().sum::<usize>()
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    const GROUPS: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_split_batch() {
        assert_eq!(split_groups(GROUPS).len(), 5);
    }

    #[test]
    fn test_split_records() {
        let split = split_passengers(&split_groups(GROUPS));
        assert_eq!(split[0].len(), 1);
        assert_eq!(split[1].len(), 3);
        assert_eq!(split[2].len(), 2);
        assert_eq!(split[3].len(), 4);
        assert_eq!(split[4].len(), 1);
    }

    #[test]
    fn test_count_unique_in_group() {
        let split = split_passengers(&split_groups(GROUPS));
        let yes_answers = count_unique_in_group(&split);
        assert_eq!(yes_answers.iter().sum::<usize>(), 11)
    }

    #[test]
    fn test_count_common_in_group() {
        let split = split_passengers(&split_groups(GROUPS));
        let common_answers = count_common_in_group(&split);
        assert_eq!(common_answers.iter().sum::<usize>(), 6)
    }
}
