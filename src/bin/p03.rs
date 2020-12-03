use anyhow::Result;
use std::io::{self, BufRead};

type Trees = Vec<Vec<bool>>;

fn main() -> Result<()> {
    let mut trees: Trees = vec![];

    for line in io::stdin().lock().lines() {
        trees.push(line?.chars().map(|c| c == '#').collect());
    }

    let width: usize = trees[0].len();
    let height: usize = trees.len();

    println!("Part 1 = {}", slope(&trees, 3, 1, width, height));

    let p2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| slope(&trees, *x, *y, width, height))
        .product();

    println!("Part 2 = {}", p2);

    Ok(())
}

fn slope(trees: &Trees, xs: usize, ys: usize, w: usize, h: usize) -> usize {
    let end = h / ys - 1;

    (0..end)
        .map(|n| (n * xs, n * ys))
        .filter(|&(x, y)| trees[y][x % w])
        .count()
}
