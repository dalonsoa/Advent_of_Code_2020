use regex::Regex;
use std::collections::HashMap;
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

/// Read the colours and their contents in two hash maps
///
/// One links the bags with its potential contents. The other links each colour with
/// those bags that can contain them.
fn get_policies(
    policies: Vec<String>,
) -> (
    HashMap<String, HashMap<String, usize>>,
    HashMap<String, Vec<String>>,
) {
    let mut contains: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut is_contained_by: HashMap<String, Vec<String>> = HashMap::new();
    let rfirst = Regex::new(r"^(\w+\s\w+) bags contain").unwrap();
    let rcontained = Regex::new(r"(\d+) (\w+\s\w+) bag?").unwrap();

    for pol in policies.iter() {
        let color = rfirst.captures(pol).unwrap()[1].to_string();
        let contents = contains.entry(color.clone()).or_insert(HashMap::new());

        for bags in rcontained.captures_iter(pol) {
            contents
                .entry(bags[2].to_string())
                .or_insert(bags[1].parse().unwrap());

            let contained = is_contained_by
                .entry(bags[2].to_string())
                .or_insert(Vec::new());
            if !contained.contains(&color) {
                contained.push(color.clone())
            }
        }
    }
    return (contains, is_contained_by);
}

/// Provide a list of color that can eventually contain the input color
fn who_contains_me(color: &str, is_contained_by: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut colors = is_contained_by[color].clone();
    for c in is_contained_by[color].iter() {
        if is_contained_by.contains_key(c) {
            colors.extend(who_contains_me(c, &is_contained_by))
        }
    }
    colors.sort_by(|a, b| a.cmp(b));
    colors.dedup();
    return colors;
}

/// Count all individual bags that can fit within a bag
fn how_many_bags(color: &str, contains: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut nbags = 1;
    for (c, num) in contains[color].iter() {
        nbags += num * how_many_bags(&c, &contains);
    }
    return nbags;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (contain, is_contained_by) = get_policies(read_file(&args));
    let color = "shiny gold";

    // How many colors can store shiny gold?
    let contained_by = who_contains_me(&color, &is_contained_by);
    println!(
        "How many colors can store shiny gold? - {}",
        contained_by.len()
    );

    // How many total bags need to fit within a shiny gold one?
    // We need to remove 1 to avoid coiunting the shiny one itself
    let total_bags = how_many_bags(&color, &contain) - 1;
    println!(
        "How many total bags need to fit within a shiny gold one? - {}",
        total_bags
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    const RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const RULES_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_get_policies() {
        let raw = RULES.split("\n").map(|s| s.to_string()).collect();
        let (contain, is_contained_by) = get_policies(raw);
        assert_eq!(contain.len(), 9);
        assert_eq!(is_contained_by.len(), 7);
    }

    #[test]
    fn test_who_contains_me() {
        let raw = RULES.split("\n").map(|s| s.to_string()).collect();
        let (_, is_contained_by) = get_policies(raw);
        let color = "shiny gold";
        let contained_by = who_contains_me(&color, &is_contained_by);
        assert_eq!(contained_by.len(), 4)
    }

    #[test]
    fn test_how_many_bags() {
        let raw = RULES_2.split("\n").map(|s| s.to_string()).collect();
        let (contain, _) = get_policies(raw);
        let color = "shiny gold";
        let total_bags = how_many_bags(&color, &contain) - 1;
        assert_eq!(total_bags, 126)
    }
}
