use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let nums: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<_,_>>()?;

    println!("Part 1 = {}", process(14, &nums, 2020));

    println!("Part 2 = {}", process(14, &nums, 30000000));

    Ok(())
}

fn process(mut last: usize, data: &Vec<usize>, limit: usize) -> usize {
    let mut seen: HashMap<usize, usize> = data
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .map(|(a, b)| (*b, a + 1))
        .collect();

    let len = data.len();

    for i in len..limit {
        let prev = *seen.get(&last).unwrap_or(&i);
        seen.insert(last, i);
        last = i - prev;
    }

    last
}

