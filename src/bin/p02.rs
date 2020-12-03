use anyhow::{anyhow, Result};
use regex::Regex;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;

    let mut passwords = vec![];

    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(&line?) {
            passwords.push((
                caps[1].parse()?,
                caps[2].parse()?,
                caps[3].chars().nth(0).ok_or(anyhow!("wtf!"))?,
                caps[4].chars().collect(),
            ));
        }
    }

    println!("Part 1 = {}", process(&passwords, &is_valid_p1));
    println!("Part 2 = {}", process(&passwords, &is_valid_p2));

    Ok(())
}

type Password = (usize, usize, char, Vec<char>);

fn is_valid_p1(pw: &Password) -> bool {
    let count = pw.3.iter().filter(|&x| *x == pw.2).count();

    count >= pw.0 && count <= pw.1
}

fn is_valid_p2(pw: &Password) -> bool {
    let idx1 = pw.3[pw.0 - 1];
    let idx2 = pw.3[pw.1 - 1];

    idx1 == pw.2 && idx2 != pw.2 || idx1 != pw.2 && idx2 == pw.2
}

fn process(passwords: &Vec<Password>, f: &dyn Fn(&Password) -> bool) -> usize {
    passwords.iter().filter(|x| f(x)).count()
}
