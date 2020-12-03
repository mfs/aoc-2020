use anyhow::Result;
use std::io::{self, BufRead};

type Trees = Vec<Vec<bool>>;

fn main() -> Result<()> {
    let mut trees: Trees = vec![];

    for line in io::stdin().lock().lines() {
        trees.push(line?.chars().map(|c| c == '#').collect());
    }

    let width = trees[0].len();
    let height = trees.len();

    println!("Part 1 = {}", slope(&trees, 3, 1, width, height));

    let p2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| slope(&trees, *x, *y, width, height))
        .product();

    println!("Part 2 = {}", p2);

    Ok(())
}

fn slope(trees: &Trees, dx: usize, dy: usize, w: usize, h: usize) -> usize {
    let end = h / dy - 1;

    (0..end)
        .map(|n| (n * dx, n * dy))
        .filter(|&(x, y)| trees[y][x % w])
        .count()
}
