use anyhow::Result;
use regex::Regex;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut directions: Vec<(char, i64)> = vec![];

    let re = Regex::new(r"([NSEWLRF])(\d+)")?;

    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(&line?) {
            directions.push((caps[1].chars().next().unwrap(), caps[2].parse()?));
        }
    }

    println!("Part 1 = {}", p1(&directions));
    println!("Part 2 = {}", p2(&directions));

    Ok(())
}

fn rotate_cw(pt: &(i64, i64), angle: i64) -> (i64, i64) {
    match angle {
        0 => *pt,
        90 | -270 => (pt.1, -pt.0),
        180 | -180 => (-pt.0, -pt.1),
        270 | -90 => (-pt.1, pt.0),
        _ => panic!(),
    }
}

fn p1(directions: &Vec<(char, i64)>) -> i64 {
    let mut ship = (0i64, 0i64);

    let mut dir = (1i64, 0i64);

    for (i, d) in directions {
        match i {
            'N' => ship.1 += d,
            'S' => ship.1 -= d,
            'E' => ship.0 += d,
            'W' => ship.0 -= d,
            'F' => ship = (ship.0 + dir.0 * d, ship.1 + dir.1 * d),
            'R' => dir = rotate_cw(&dir, *d),
            'L' => dir = rotate_cw(&dir, -*d),
            _ => panic!(),
        }
    }

    ship.0.abs() + ship.1.abs()
}

fn p2(directions: &Vec<(char, i64)>) -> i64 {
    let mut ship = (0, 0);

    let mut wp = (10, 1);

    for (i, d) in directions {
        match i {
            'N' => wp.1 += d,
            'S' => wp.1 -= d,
            'E' => wp.0 += d,
            'W' => wp.0 -= d,
            'F' => ship = (ship.0 + d * wp.0, ship.1 + d * wp.1),
            'R' => wp = rotate_cw(&wp, *d),
            'L' => wp = rotate_cw(&wp, -*d),
            _ => panic!(),
        }
    }

    ship.0.abs() + ship.1.abs()
}
