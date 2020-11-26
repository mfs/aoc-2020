use anyhow::Result;

fn main() -> Result<()> {
    println!("Advent of Code 2020!");

    /* process stdin as lines
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        println!("{}", line?);
    }
    */

    /* process stdin as chars (ascii only)
    use std::io::{self, Read};
    for byte in io::stdin().lock().bytes() {
        println!("{}", byte? as char);
    }
    */

    /* process stdin as lines of space separated ints
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        let x: Vec<u64> = line?
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        println!("{:?}", x);
    }
    */

    Ok(())
}
