use anyhow::Result;
use std::io::{self, BufRead};

type Map = Vec<Vec<char>>;

fn main() -> Result<()> {
    let mut map: Map = vec![];

    for line in io::stdin().lock().lines() {
        map.push(line?.chars().collect());
    }

    println!("Part 1 = {}", process(&map, 4, false));

    println!("Part 2 = {}", process(&map, 5, true));

    Ok(())
}

fn process(map: &Map, occupied: i64, ray: bool) -> usize {
    let mut primary = map.clone();
    let mut secondary = map.clone();

    loop {
        step(&primary, &mut secondary, occupied, ray);

        if primary == secondary {
            return primary.iter().flatten().filter(|&x| x == &'#').count();
        }

        let tmp = primary;
        primary = secondary;
        secondary = tmp;
    }
}

fn step(map: &Map, next: &mut Map, occupied: i64, ray: bool) {
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let n = count_occupied_neighbours(x as i64, y as i64, map, ray);
            if c == 'L' && n == 0 {
                next[y][x] = '#';
            } else if c == '#' && n >= occupied {
                next[y][x] = 'L';
            } else {
                next[y][x] = c;
            }
        }
    }
}

#[rustfmt::skip]
const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
];

fn count_occupied_neighbours(x: i64, y: i64, map: &Map, ray: bool) -> i64 {
    let mut count = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        for m in 1.. {
            let nx = x + dx * m;
            let ny = y + dy * m;
            if nx < 0 || nx >= map[0].len() as i64 || ny < 0 || ny >= map.len() as i64 {
                break;
            }

            match map[ny as usize][nx as usize] {
                'L' => break,
                '#' => { count += 1; break; }
                _ => { }
            }

            if !ray {
                break;
            }
        }
    }

    count
}
