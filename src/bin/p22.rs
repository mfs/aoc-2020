use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut player1: Vec<usize> = vec![];
    let mut player2: Vec<usize> = vec![];

    let mut player = &mut player1;

    for line in io::stdin().lock().lines() {
        let line = line?;
        match &*line {
            "Player 1:" | "" => {},
            "Player 2:" => player = &mut player2,
            _ => player.push(line.trim().parse()?),
        }
    }

    println!("Part 1 = {}", combat(player1.clone(), player2.clone(), false).1);

    println!("Part 2 = {}", combat(player1, player2, true).1);

    Ok(())
}

// return value (winner, checksum)
fn combat(mut player1: Vec<usize>, mut player2: Vec<usize>, recurse: bool) -> (usize, usize) {
    let mut history = HashSet::new();

    loop {
        // check history if recursing
        if !history.insert((player1.clone(), player2.clone())) && recurse {
            return (1, checksum(&player1));
        }

        // check for win
        if player1.is_empty() {
            return (2, checksum(&player2));
        } else if player2.is_empty() {
            return (1, checksum(&player1));
        }

        // draw cards
        let p1_card = player1.remove(0);
        let p2_card = player2.remove(0);

        let mut winner = 1;

        if player1.len() >= p1_card && player2.len() >= p2_card && recurse {
            // recursive combat
            let player1_new = player1[..p1_card].to_vec();
            let player2_new = player2[..p2_card].to_vec();

            winner = combat(player1_new, player2_new, recurse).0;
        } else {
            // standard combat
            if p2_card > p1_card {
                winner = 2;
            }
        }

        if winner == 1 {
            player1.push(p1_card);
            player1.push(p2_card);
        } else {
            player2.push(p2_card);
            player2.push(p1_card);
        }
    }
}

fn checksum(deck: &Vec<usize>) -> usize {
    deck.iter().rev().enumerate().map(|(i, &s)| (i + 1) * s).sum()
}
