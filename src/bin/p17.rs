use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;

type Cube = (i64, i64, i64, i64);

fn main() -> Result<()> {

    let mut cubes: HashSet<Cube> = HashSet::new();

    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' {
                cubes.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    println!("Part 1 = {}", process(&mut cubes.clone(), 0));

    println!("Part 2 = {}", process(&mut cubes, 1));

    Ok(())
}

fn process(cubes: &mut HashSet<Cube>, wm: i64) -> usize {
    for _ in 0..6 {
        let mut nc = HashSet::new();

        let (min, max) = min_max(&cubes);

        for w in min.3..=max.3 {
            for z in min.2..=max.2 {
                for y in min.1..=max.1 {
                    for x in min.0..=max.0 {
                        let n = count_active((x, y, z, w * wm), &cubes, wm);

                        if cubes.contains(&(x, y, z, w * wm)) && (n == 2 || n == 3) {
                            nc.insert((x, y, z, w * wm));
                        } else if !cubes.contains(&(x, y, z, wm * w)) && n == 3 {
                            nc.insert((x, y, z, w * wm));
                        }
                    }
                }
            }
        }

        *cubes = nc;
    }

    cubes.iter().count()
}


fn min_max(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    let mut xs: Vec<_> = cubes.iter().map(|c| c.0).collect();
    let mut ys: Vec<_> = cubes.iter().map(|c| c.1).collect();
    let mut zs: Vec<_> = cubes.iter().map(|c| c.2).collect();
    let mut ws: Vec<_> = cubes.iter().map(|c| c.2).collect();

    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();
    ws.sort_unstable();

    let last = xs.len() - 1;

    let min = (xs[0] - 1, ys[0] - 1, zs[0] - 1, ws[0] - 1);
    let max = (xs[last] + 1, ys[last] + 1, zs[last] + 1, ws[last] + 1);

    (min, max)
}

fn count_active(pt: Cube, cubes: &HashSet<Cube>, wm: i64) -> usize {
    neighbours(pt, wm)
        .iter()
        .filter(|x| cubes.contains(&x))
        .count()
}

fn neighbours(pt: Cube, wm: i64) -> HashSet<Cube> {
    let mut n = HashSet::new();

    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x != 0 || y != 0 || z != 0 || (wm * w != 0) {
                        n.insert((pt.0 + x, pt.1 + y, pt.2 + z, (pt.3 + w) * wm));
                    }
                }
            }
        }
    }

    n
}
