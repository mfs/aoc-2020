use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashMap;

type Map = HashMap<(i64, i64), char>;

fn main() -> Result<()> {
    let mut map = HashMap::new();

    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            map.insert((x as i64, y as i64), c);
        }
    }

    println!("Part 1 = {}", process(&map, 4, false));

    println!("Part 2 = {}", process(&map, 5, true));

    Ok(())
}

fn process(map: &Map, occupied: i64, ray: bool) -> usize {
    let mut map = map.clone();

    loop {
        let new = step(&map, occupied, ray);

        if map == new {
            return map.values().filter(|&x| *x == '#').count();
        }

        map = new;
    }
}

fn step(map: &Map, occupied: i64, ray: bool) -> Map {
    let mut new = HashMap::new();

    for (&(x, y), &c) in map {
        let n = count_occupied_neighbours(x, y, map, ray);
        if c == 'L' && n == 0 {
            new.insert((x, y), '#');
        } else if c == '#' && n >= occupied {
            new.insert((x, y), 'L');
        } else {
            new.insert((x, y), c);
        }
    }

    new
}

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
];

fn count_occupied_neighbours(x: i64, y: i64, map: &Map, ray: bool) -> i64 {
    let mut count = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        for m in 1.. {
            match map.get(&(x + dx * m, y + dy * m)) {
                None => break,
                Some('L') => break,
                Some('#') => { count += 1; break; }
                _ => {},
            }

            if !ray {
                break;
            }
        }
    }

    count
}
