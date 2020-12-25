use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut numbers: Vec<u64> = vec![];
    for line in io::stdin().lock().lines() {
        numbers.push(line?.parse()?);
    }

    let card = numbers[0];
    let door = numbers[1];

    let card_key = calc_key(card, find_loop_size(door, 7));
    let door_key = calc_key(door, find_loop_size(card, 7));

    assert!(card_key == door_key);

    println!("Part 1 = {}", card_key);

    Ok(())
}

fn find_loop_size(public_key: u64, subject: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;

    while value != public_key {
        value = value * subject;
        value %= 20201227;
        loop_size += 1;
    }

    loop_size
}

fn calc_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1u64;

    for _ in 0..loop_size {
        value = value * public_key;
        value %= 20201227;
    }

    value
}
