use anyhow::{Result, anyhow};
use std::io::{self, BufRead};
use std::iter::Iterator;

fn main() -> Result<()> {

    let mut seats: Vec<u64> = vec![];

    for line in io::stdin().lock().lines() {
        seats.push(calc_id(&line?));
    }

    println!("Part 1 = {}", seats.iter().max().ok_or(anyhow!("wtf!"))?);

    seats.sort();

    for w in seats.windows(2).filter(|w| w[1] == w[0] + 2) {
        println!("Part 2 = {}", w[0] + 1);
    }

    Ok(())
}

fn bsp(mut min: u64, mut max: u64, chars: &mut dyn Iterator<Item = char>) -> u64 {
    for c in chars {
        match c {
            'F'|'L' => { max = (max + min) / 2 },
            'B'|'R' => { min = (max + min) / 2 + 1 },
            _ => panic!(),
        }
    }

    min
}

fn calc_id(pass: &str) -> u64 {
    let row = bsp(0, 127, &mut pass.chars().take(7));
    let seat = bsp(0, 7, &mut pass.chars().skip(7));

    row * 8 + seat
}
