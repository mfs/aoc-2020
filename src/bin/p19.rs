use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    Seq(Vec<u64>),
    Alt((Vec<u64>, Vec<u64>)),
    Let(char),
}

fn main() -> Result<()> {
    let mut rules = HashMap::new();
    let mut messages = vec![];

    let re_seq = Regex::new(r"^(\d+): ((?:\d+\s*)+)$")?;
    let re_alt = Regex::new(r"^(\d+): ((?:\d+\s*)+) \| ((?:\d+\s*)+)$")?;
    let re_let = Regex::new(r#"^(\d+): "(.)"$"#)?;

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(":") {
            if let Some(caps) = re_seq.captures(&line) {
                let id: u64 = caps[1].parse()?;
                let v: Vec<u64> = caps[2]
                    .split(' ')
                    .map(|x| x.parse())
                    .collect::<Result<_,_>>()?;

                rules.insert(id, Rule::Seq(v));
                continue;
            }

            if let Some(caps) = re_alt.captures(&line) {
                let id: u64 = caps[1].parse()?;
                let v0: Vec<u64> = caps[2]
                    .split(' ')
                    .map(|x| x.parse())
                    .collect::<Result<_,_>>()?;
                let v1: Vec<u64> = caps[3]
                    .split(' ')
                    .map(|x| x.parse())
                    .collect::<Result<_,_>>()?;

                rules.insert(id, Rule::Alt((v0, v1)));
                continue;
            }

            if let Some(caps) = re_let.captures(&line) {
                let id: u64 = caps[1].parse()?;
                let letter: char = caps[2].chars().next().unwrap();
                rules.insert(id, Rule::Let(letter));
                continue;
            }
        } else if line.len() > 0 {
            messages.push(line);
        }
    }

    let msg_re = Regex::new(&format!("^{}$", build_regex(&rules, 0, 0)))?;

    let p1 = messages.iter().filter(|x| msg_re.is_match(x)).count();

    println!("{}", p1);

    // part two
    rules.insert(8, Rule::Alt((vec![42], vec![42, 8])));
    rules.insert(11, Rule::Alt((vec![42, 31], vec![42, 11, 31])));

    let msg_re = Regex::new(&format!("^{}$", build_regex(&rules, 0, 0)))?;

    let p2 = messages.iter().filter(|x| msg_re.is_match(x)).count();

    println!("{}", p2);

    Ok(())
}

fn build_regex(rules: &HashMap<u64, Rule>, id: u64, depth: u64) -> String {
    if depth == 15 {
        return "".to_owned();
    }
    match &rules[&id] {
        Rule::Seq(x) => {
            return x.iter().map(|&x| build_regex(rules, x, depth + 1)).collect();
        },
        Rule::Let(x) => {
            return x.to_string();
        },
        Rule::Alt((a, b)) => {
            let va: String = a.iter().map(|&x| build_regex(rules, x, depth + 1)).collect();
            let vb: String = b.iter().map(|&x| build_regex(rules, x, depth + 1)).collect();
            return format!("({}|{})", va, vb);
        },
    }
}

