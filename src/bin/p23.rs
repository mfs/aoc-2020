use anyhow::Result;
use std::collections::{VecDeque,HashMap};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let cups: Vec<i64> = buffer
        .trim()
        .chars()
        .map(|x| x as i64 - '0' as i64)
        .collect();

    println!("Part 1 = {}", p1(&cups));

    println!("Part 2 = {}", p2(&cups));

    Ok(())
}

fn p1(cups: &Vec<i64>) -> String {
    let mut cups: VecDeque<i64> = cups.iter().copied().collect();
    let mut three = [0i64; 3];

    for _ in 0..100 {
        // current cup is [0]
        let current = cups[0];
        cups.rotate_left(1);

        // three cups now at start
        three[0] = cups.pop_front().unwrap();
        three[1] = cups.pop_front().unwrap();
        three[2] = cups.pop_front().unwrap();

        // find destination
        let mut destination = dec(current, 9);

        while three.contains(&destination) {
            destination = dec(destination, 9);
        }

        // find index of destination (this is the main reason tihs didn't work for part 2)
        let idx = cups.iter().position(|&x| x == destination).unwrap();

        // insert three cups
        cups.insert(idx + 1, three[0]);
        cups.insert(idx + 2, three[1]);
        cups.insert(idx + 3, three[2]);
    }

    cups
        .iter()
        .cycle()
        .skip_while(|&x| x != &1)
        .skip(1)
        .take(8)
        .map(|x| format!("{}", x))
        .collect()
}

fn p2(cups: &Vec<i64>) -> i64 {
    let sequence = cups
        .iter()
        .copied()
        .chain((10..=1_000_000).into_iter());

    let mut map: HashMap<i64, i64> = sequence
        .clone()
        .zip(sequence.skip(1))
        .collect();

    map.insert(1_000_000, cups[0]);

    let mut current = cups[0];

    let mut three = [0i64; 3];

    for _ in 0..10_000_000 {
        // remove three
        three[0] = map[&current];
        three[1] = map[&three[0]];
        three[2] = map[&three[1]];
        map.insert(current, map[&three[2]]);

        // find destination
        let mut destination = dec(current, 1_000_000);

        while three.contains(&destination) {
            destination = dec(destination, 1_000_000);
        }

        // insert three back in
        map.insert(three[2], map[&destination]);
        map.insert(destination, three[0]);

        // move forward
        current = map[&current];
    }

    map[&1] * map[&map[&1]]
}

fn dec(n: i64, max: i64) -> i64 {
   match n {
        1 => max,
        _ => n - 1,
   }
}
