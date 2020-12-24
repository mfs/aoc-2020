use anyhow::Result;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashSet;

type Tile = (i64, i64, i64);

fn main() -> Result<()> {
    let re = Regex::new(r"e|se|sw|w|nw|ne")?;

    let mut flipped: Vec<Vec<String>> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let mut tile: Vec<String> = vec![];
        for d in re.captures_iter(&line) {
            tile.push(d.get(0).unwrap().as_str().to_owned());
        }
        flipped.push(tile);
    }

    // part 1
    let mut map = HashSet::new();
    let mut start: (i64, i64, i64);

    for moves in &flipped {
        start = (0, 0, 0);
        for m in moves {
            let s: &str = &m;

            match s {
                "e" => { start.0 += 1; start.1 -= 1; },
                "w" => { start.0 -= 1; start.1 += 1; },
                "ne" => { start.0 += 1; start.2 -= 1; },
                "nw" => { start.1 += 1; start.2 -= 1; },
                "se" => { start.1 -= 1; start.2 += 1; },
                "sw" => { start.0 -= 1; start.2 += 1; },
                _ => panic!(),
            }
        }
        if map.contains(&start) {
            map.remove(&start);
        } else {
            map.insert(start);
        }
    }

    println!("Part 1 = {}", map.len());

    // part 2
    for _ in 0..100 {
        // make new map
        let mut next_map = map.clone();

        // calc working set
        let mut working = HashSet::new();
        for t in &map {
            working.insert(*t);
            for n in neighbours(*t) {
                working.insert(n);
            }
        }

        // update tiles
        for tile in &working {
            let (_, bn) = count_neighbours(*tile, &map);
            if map.contains(tile) {
                // current tile is black
                if bn == 0 || bn > 2 {
                    next_map.remove(tile); // flip white
                }
            } else {
                // current tile is white
                if bn == 2 {
                    next_map.insert(*tile); // flip black
                }
            }
        }

        // update map
        map = next_map;
    }

    println!("Part 2 = {}", map.len());

    Ok(())
}

const NEIGHBOURS: [Tile; 6] = [
    ( 1, -1,  0), // e
    (-1,  1,  0), // w
    ( 1,  0, -1), // ne
    ( 0,  1, -1), // nw
    ( 0, -1,  1), // se
    (-1,  0,  1), // sw
];

fn neighbours(tile: Tile) -> HashSet<Tile> {
    NEIGHBOURS
        .iter()
        .map(|n| (tile.0 + n.0, tile.1 + n.1, tile.2 + n.2))
        .collect()
}

fn count_neighbours(tile: Tile, map: &HashSet<Tile>) -> (usize, usize) {
    let black = neighbours(tile)
        .iter()
        .filter(|t| map.contains(t))
        .count();

    (6 - black, black)
}

