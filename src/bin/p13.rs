use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {

    let mut lines = vec![];

    for line in io::stdin().lock().lines() {
        lines.push(line?);
    }

    let timestamp: u64 = lines[0].parse()?;

    let ids: Vec<u64> = lines[1].split(',').map(|x| x.parse().unwrap_or(0)).collect();

    println!("Part 1 = {}", p1(&ids, timestamp));

    println!("Part 2 = {}", p2(&ids));

    Ok(())
}

fn p1(ids: &Vec<u64>, timestamp: u64) -> u64 {
    for t in timestamp.. {
        for id in ids.iter().filter(|&x| *x != 0) {
            if t % id == 0 {
                let d = t - timestamp;
                return id * d;
            }
        }
    }

    0
}

// I solved this by writing code to output the equations and then pasting them into an online
// Chinese Remainder Theorom solver. I was going to implement this in code and then came across
// this code by @lizthegrey. Even after rewriting it in Rust I'm still not sure exactly how it
// works :(.
// https://github.com/lizthegrey/adventofcode/blob/2e0bc706c12a973bc2849e4d3b88ba66e9c8b2a4/2020/day13.go#L50
fn p2(ids: &Vec<u64>) -> u64 {
    let mut min = 0u64;
    let mut product = 1u64;

    for (i, v) in ids.iter().enumerate().filter(|&(_, v)| *v != 0) {
        while (min + i as u64) % * v != 0 {
            min += product;
        }
        product *= *v;
    }

    min as u64
}
