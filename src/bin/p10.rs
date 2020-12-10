use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut adapters: Vec<i64> = vec![];

    for line in io::stdin().lock().lines() {
        adapters.push(line?.parse()?);
    }

    adapters.push(0);
    adapters.sort();
    let last = adapters.last().ok_or(anyhow!("empty input"))? + 3;
    adapters.push(last);

    println!("Part 1 = {}", p1(&adapters));

    println!("Part 2 = {}", p2(&adapters)[&last]);

    Ok(())
}

fn p1(adapters: &Vec<i64>) -> usize {
    let mut diff = vec![];

    for a in adapters.iter().zip(adapters[1..].iter()) {
        diff.push(a.1 - a.0);
    }

    let ones = diff.iter().filter(|&x| x == &1).count();
    let threes = diff.iter().filter(|&x| x == &3).count();

    ones * threes
}

fn p2(adapters: &Vec<i64>) -> HashMap<i64, i64> {
    let mut paths = HashMap::new();
    paths.insert(0, 1);

    for n in adapters {
        for i in 1..4 {
            if adapters.contains(&(n + i)) {
                *paths.entry(n + i).or_insert(0) += paths[n];
            }
        }
    }

    paths
}
