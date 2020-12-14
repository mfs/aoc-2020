use anyhow::Result;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Inst {
    Mask(String),
    Mem(u64, u64),
}

fn main() -> Result<()> {
    let re_mask = Regex::new(r"^mask = ([X01]{36})$")?;
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$")?;

    let mut inst = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if let Some(caps) = re_mask.captures(&*line) {
            inst.push(Inst::Mask(caps[1].to_owned()));
        }

        if let Some(caps) = re_mem.captures(&*line) {
            inst.push(Inst::Mem(caps[1].parse()?, caps[2].parse()?));
        }
    }

    println!("Part 1 = {}", p1(&inst));

    println!("Part 2 = {}", p2(&inst));

    Ok(())
}

fn p1(inst: &Vec<Inst>) -> u64 {
    let mut mem = HashMap::new();
    let mut masks = (0u64, 0u64);

    for i in inst {
        match i {
            Inst::Mask(mask) => {
                masks = calc_masks(&mask);
            },
            Inst::Mem(addr, val) => {
                mem.insert(addr, (val | masks.0) & masks.1);
            },
        }
    }

    mem.values().sum()
}

fn calc_masks(mask: &str) -> (u64, u64) {
    let mut mask0 = 0;
    let mut mask1 = 0;

    for (i, b) in mask.chars().rev().enumerate() {
        let m: u64 = 1 << i;

        match b {
            '0' => { mask0 &= !m; mask1 &= !m; },
            '1' => { mask0 |= m; mask1 |= m; },
            'X' => { mask0 &= !m; mask1 |= m; },
            _ => panic!(),
        }
    }

    (mask0, mask1)
}

fn p2(inst: &Vec<Inst>) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = String::new();

    for i in inst {
        match i {
            Inst::Mask(m) => {
                mask = m.clone();
            },
            Inst::Mem(addr, val) => {
                let v = apply_mask(*addr, &mask);
                for a in addresses(&v) {
                    mem.insert(a, *val);
                }
            },
        }
    }

    mem.values().sum()
}

fn apply_mask(addr:  u64, mask: &str) -> String {
    let a = format!("{:036b}", addr);

    let mut out = String::new();

    for (b, m) in a.chars().zip(mask.chars()) {
        match m {
            '0' => out.push(b),
            '1' => out.push('1'),
            'X' => out.push(m),
            _   => panic!(),
        }
    }

    out
}

fn addresses(addr: &str) -> Vec<u64> {
    if !addr.contains('X') {
        return vec![u64::from_str_radix(addr, 2).unwrap()];
    }

    let v0 = addresses(&addr.replacen("X", "0", 1));
    let v1 = addresses(&addr.replacen("X", "1", 1));

    [v0, v1].concat()
}

