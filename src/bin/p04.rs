use anyhow::Result;
use std::io::{self, Read};
use regex::Regex;
use std::collections::HashMap;

type Fields<'a> = [(&'a str, regex::Regex, fn(&str, &regex::Captures) -> bool); 7];

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

    let fields: Fields = [
        ("byr", Regex::new(r"^\d{4}$")?, validate_year),
        ("iyr", Regex::new(r"^\d{4}$")?, validate_year),
        ("eyr", Regex::new(r"^\d{4}$")?, validate_year),
        ("hgt", Regex::new(r"^(\d+)(cm|in)$")?, validate_hgt),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$")?, |_, _| true),
        ("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$")?, |_, _| true),
        ("pid", Regex::new(r"^\d{9}$")?, |_, _| true )
    ];

    println!("Part 1 = {}", passports.iter().filter(|x| is_valid(x, &fields, false)).count());

    println!("Part 2 = {}", passports.iter().filter(|x| is_valid(x, &fields, true)).count());

    Ok(())
}

fn validate_year(field: &str, cap: &regex::Captures) -> bool {
    let year: u64 = cap[0].parse().unwrap();

    match field {
        "byr" => year >= 1920 && year <= 2002,
        "iyr" => year >= 2010 && year <= 2020,
        "eyr" => year >= 2020 && year <= 2030,
        _ => panic!(),
    }
}

fn validate_hgt(_: &str, cap: &regex::Captures) -> bool {
    let h: u64 = cap[1].parse().unwrap();

    (&cap[2] == "cm" && h >= 150 && h <= 193) || (&cap[2] == "in" && h >= 59 && h <= 76)
}

fn is_valid(p: &HashMap<String, String>, fields: &Fields , strict: bool) -> bool {
    for (k, re, is_valid_fn) in fields.iter() {
        if !p.contains_key(k.to_owned()) {
            return false
        }

        if !strict {
            continue;
        }

        if let Some(cap) = re.captures(&p[k.to_owned()]) {
            if !is_valid_fn(k, &cap) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
