use anyhow::Result;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

type Field = (String, u64, u64, u64, u64);
type Ticket = Vec<u64>;

fn main() -> Result<()> {
    let re_field = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)")?;

    let mut fields: Vec<Field> = vec![];
    let mut tickets = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(" or ") {
            if let Some(caps) = re_field.captures(&line) {
                fields.push((
                    caps[1].to_owned(),
                    caps[2].parse()?,
                    caps[3].parse()?,
                    caps[4].parse()?,
                    caps[5].parse()?,
                ));
            }
        } else if line.contains(",") {
            let v: Vec<u64> = line.trim().split(",").map(|x| x.parse()).collect::<Result<_,_>>()?;
            tickets.push(v);
        }
    }

    let my_ticket = tickets.remove(0);

    let mut valid_tickets = vec![];

    println!("Part 1 = {}", p1(&tickets, &fields, &mut valid_tickets));

    let mul: u64 = p2(&valid_tickets, &fields)
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_,&v)| my_ticket[v])
        .product();

    println!("Part 2 = {}", mul);

    Ok(())
}

fn p1(tickets: &Vec<Ticket>, fields: &Vec<Field>, valid_tickets: &mut Vec<Ticket>) -> u64 {
    let mut invalids = vec![];

    for t in tickets {
        let inv_fields = invalid_fields(t, &fields);
        if inv_fields.len() == 0 {
            valid_tickets.push(t.clone());
        }
        invalids.push(inv_fields);
    }

    invalids.iter().flatten().sum::<u64>()
}

fn invalid_fields(ticket: &Ticket, fields: &Vec<Field>) -> Vec<u64> {
    let mut result = vec![];
    'outer: for &t in ticket {
        for field in fields {
            if is_valid(t, &field) {
                continue 'outer;
            }
        }
        result.push(t);
    }

    result
}

fn p2(tickets: &Vec<Ticket>, fields: &Vec<Field>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    for _ in 0..fields.len() { // loop once for each field
        for field in fields { // loop over remaining fields/indices
            if map.contains_key(&field.0) {
                continue; // already found this field's index
            }

            let mut indices = vec![];
            'outer: for idx in 0..fields.len() {
                if map.values().find(|&x| *x == idx).is_some() {
                    continue; // already associated this index with a field
                }

                for t in tickets {
                    if !is_valid(t[idx as usize], &field) {
                        continue 'outer; // not valid
                    }
                }
                indices.push(idx); // this index is valid
            }

            if indices.len() == 1 { // only one found, mark off
                map.insert(field.0.clone(), indices[0]);
            }
        }
    }

    map
}

fn is_valid(x: u64, field: &Field) -> bool {
    x >= field.1 && x <= field.2 || x >= field.3 && x <= field.4
}
