use anyhow::Result;
use std::io::{self, Read};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s)?;

    let re = Regex::new(r"([a-z]{3}):([^\s]+)")?;

    let mut passports = vec![];

    for e in s.split("\n\n") {
        let mut passport: HashMap<String, String> = HashMap::new();

        for c in re.captures_iter(e) {
            passport.insert(c[1].to_owned(), c[2].to_owned());
        }

        passports.push(passport);
    }

    println!("Part 1 = {}", passports.iter().filter(|x| is_valid(x, false)).count());

    println!("Part 2 = {}", passports.iter().filter(|x| is_valid(x, true)).count());

    Ok(())
}

fn validate(_: &regex::Captures) -> bool {
    true
}

fn validate_byr(cap: &regex::Captures) -> bool {
    let year: u64 = cap[0].parse().unwrap();

    year >= 1920 && year <= 2002
}

fn validate_iyr(cap: &regex::Captures) -> bool {
    let year: u64 = cap[0].parse().unwrap();

    year >= 2010 && year <= 2020
}

fn validate_eyr(cap: &regex::Captures) -> bool {
    let year: u64 = cap[0].parse().unwrap();

    year >= 2020 && year <= 2030
}

fn validate_hgt(cap: &regex::Captures) -> bool {
    let h: u64 = cap[1].parse().unwrap();

    (&cap[2] == "cm" && h >= 150 && h <= 193) || (&cap[2] == "in" && h >= 59 && h <= 76)
}

fn is_valid(p: &HashMap<String, String>, strict: bool) -> bool {
    let fields: [(&str, regex::Regex, fn(&regex::Captures) -> bool); 7] = [
        ("byr", Regex::new(r"^\d{4}$").unwrap(), validate_byr),
        ("iyr", Regex::new(r"^\d{4}$").unwrap(), validate_iyr),
        ("eyr", Regex::new(r"^\d{4}$").unwrap(), validate_eyr),
        ("hgt", Regex::new(r"^(\d+)(cm|in)$").unwrap(), validate_hgt),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap(), validate),
        ("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap(), validate),
        ("pid", Regex::new(r"^\d{9}$").unwrap(), validate)
    ];

    for (k, re, is_valid_fn) in fields.iter() {
        if !p.contains_key(k.to_owned()) {
            return false
        }

        if !strict {
            continue;
        }

        if let Some(cap) = re.captures(&p[k.to_owned()]) {
            if !is_valid_fn(&cap) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
