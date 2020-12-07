use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut bags = HashMap::new(); // maps outer bag to vec of (inner bag, count)
    let mut bags_reverse = vec![]; // maps inner bag to outer bag vec of (inner, outer)

    let re = Regex::new(r"(\d+) (.*) bags?")?;

    for line in io::stdin().lock().lines() {
        let line = line?;
        let e: Vec<&str> = line.split(" bags contain ").collect();

        let mut contents = vec![];
        for bag in e[1].trim().split(", ") {
            if let Some(caps) = re.captures(bag) {
                contents.push((caps[2].to_owned(), caps[1].parse()?));
                bags_reverse.push((caps[2].to_owned(), e[0].to_owned()));
            }
        }
        bags.insert(e[0].to_owned(), contents);
    }

    println!("Part 1 = {}", contains(&bags_reverse, "shiny gold").len());

    println!("Part 2 = {}", count_bags(&bags, "shiny gold"));

    Ok(())
}

fn contains<'a>(bags: &'a Vec<(String, String)>, desc: &str) -> HashSet<&'a str> {
    let mut h: HashSet<&str> = HashSet::new();

    for o in bags.iter().filter(|(a, _)| a == desc) {
        h.insert(&o.1);
        for x in &contains(bags, &o.1) {
            h.insert(x);
        }
    }

    h
}

fn count_bags(bags: &HashMap<String, Vec<(String, u64)>>, desc: &str) -> u64 {
    bags[desc]
        .iter()
        .map(|(col, n)| n * (count_bags(&bags, col) + 1))
        .sum()
}
