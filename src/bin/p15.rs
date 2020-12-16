use anyhow::Result;
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
    let mut seen: Vec<i32> = vec![-1; limit];

    for (a, b) in data.iter().enumerate().rev() {
        seen[*b] = (a + 1) as i32
    }

    let len = data.len() as i32;

    for i in len..limit as i32 {
        let prev = match seen[last] {
            -1 => i,
            _  => seen[last],
        };
        seen[last] = i;
        last = (i - prev) as usize;
    }

    last
}
