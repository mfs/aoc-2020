use anyhow::Result;
use std::io::{self, Read};
use std::collections::HashSet;

type Groups = Vec<Vec<Vec<char>>>;

fn main() -> Result<()> {

    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s)?;

    let mut groups: Groups = vec![];

    for group in s.trim().split("\n\n") {
        groups.push(
            group
            .split("\n")
            .map(|x| x.chars().collect())
            .collect()
        );
    }

    println!("Part 1 = {}", p1(&groups));

    println!("Part 2 = {}", p2(&groups));

    Ok(())
}

fn p1(groups: &Groups) -> usize {
    let mut count = 0;

    for group in groups {
        let questions = group
            .iter()
            .flatten()
            .map(|x| *x)
            .collect::<HashSet<char>>()
            .len();
        count += questions;
    }

    count
}

fn p2(groups: &Groups) -> usize {
    let mut count = 0;

    for group in groups {
        for c in 'a'..='z' {
            if group.iter().all(|x| x.contains(&c)) {
                count += 1;
            }
        }
    }

    count
}
