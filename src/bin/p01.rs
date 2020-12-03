use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut numbers = vec![];

    for line in io::stdin().lock().lines() {
        numbers.push(line?.parse()?);
    }

    p1(&numbers);
    p2(&numbers);

    Ok(())
}

fn p1(numbers: &Vec<u64>) {
    for n in numbers {
        if numbers.contains(&(2020 - *n)) {
            println!("Part 1 = {}", n * (2020 - n));
            break;
        }
    }
}

fn p2(numbers: &Vec<u64>) {
    let len = numbers.len();

    for a in 0..len {
        for b in a + 1..len {
            for c in b + 1..len {
                if numbers[a] + numbers[b] + numbers[c] == 2020 {
                    println!("Part 2 = {}", numbers[a] * numbers[b] * numbers[c]);
                }
            }
        }
    }
}
