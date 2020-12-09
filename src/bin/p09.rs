use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut numbers: Vec<u64> = vec![];

    for line in io::stdin().lock().lines() {
        numbers.push(line?.parse()?);
    }

    let p1 = p1(&numbers);
    println!("Part 1 = {}", p1);

    println!("Part 2 = {}", p2(&numbers, p1));

    Ok(())
}

const SIZE: usize = 25;

fn p1(numbers: &Vec<u64>) -> u64 {
    'outer: for window in numbers.windows(SIZE + 1) {
        let preamble = &window[..SIZE];
        let number = window[SIZE];

        for n in preamble.iter() {
            if preamble.contains(&(number - n)) {
                continue 'outer;
            }
        }

        return number;
    }

    0
}

fn p2(numbers: &Vec<u64>, target: u64) -> u64 {
    for win_size in 2.. {
        for win in numbers.windows(win_size) {
            if target == win.iter().map(|x| *x).sum() {
                let min = win.iter().min().unwrap();
                let max = win.iter().max().unwrap();
                return min + max;
            }
        }
    }

    0
}
