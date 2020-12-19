use anyhow::{anyhow, Result};
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Token {
    Number(u64),
    Plus,
    Star,
    LParen,
    RParen,
}

fn main() -> Result<()> {
    let mut expressions = vec![];

    for line in io::stdin().lock().lines() {
        let mut tokens = vec![];
        for c in line?.trim().chars() {
            match c {
                ' ' => {},
                '+' => tokens.push(Token::Plus),
                '*' => tokens.push(Token::Star),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '0'..='9' => tokens.push(Token::Number(c as u64 - '0' as u64)),
                _ => panic!("Unexpected character: [{}]", c),
            }
        }

        expressions.push(tokens);
    }

    let mut precedence = HashMap::new();
    precedence.insert(Token::Plus, 100);
    precedence.insert(Token::Star, 100);

    println!("Part 1 = {}", expressions.iter().map(|e| parse(e, &precedence)).sum::<u64>());

    precedence.insert(Token::Plus, 200);

    println!("Part 2 = {}", expressions.iter().map(|e| parse(e, &precedence)).sum::<u64>());

    Ok(())
}

// http://math.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
fn parse(exp: &Vec<Token>, precedence: &HashMap<Token, u64>) -> u64 {
    let mut operators: Vec<Token> = vec![];
    let mut postfix: Vec<Token> = vec![];

    for &t in exp {
        match t {
            Token::Number(_) => postfix.push(t), // 1. print operand
            Token::LParen => operators.push(t), // 2. left paren
            Token::RParen => { // 3. right paren
                loop {
                    match operators.pop() {
                        None => panic!(),
                        Some(Token::LParen) => break,
                        Some(x) => postfix.push(x),
                    }
                }
            },
            Token::Star | Token::Plus => {
                if operators.is_empty() || *operators.last().unwrap() == Token::LParen {
                    operators.push(t); // 4. operator
                } else if precedence[&t] >= precedence[operators.last().unwrap()] {
                        // 5. push if greater precedence than top of operator stack
                        operators.push(t);
                } else { // 6. less than or lower precedence
                    loop {
                        match operators.last() {
                            None => break,
                            Some(Token::LParen) => break,
                            Some(_) => postfix.push(operators.pop().unwrap()),
                        }
                    }
                    operators.push(t);
                }
            },
        }
    }

    // 7. collect remaining operators
    while let Some(o) = operators.pop() {
        postfix.push(o);
    }

    process_postfix(&postfix).unwrap()
}

fn process_postfix(tokens: &Vec<Token>) -> Result<u64> {
    let mut stack: Vec<u64> = vec![];
    for t in tokens {
        match t {
            Token::Number(x) => stack.push(*x),
            Token::Plus => {
                let b = stack.pop().ok_or(anyhow!("stack underflow"))?;
                let a = stack.pop().ok_or(anyhow!("stack underflow"))?;
                stack.push(a + b);
            },
            Token::Star => {
                let b = stack.pop().ok_or(anyhow!("stack underflow"))?;
                let a = stack.pop().ok_or(anyhow!("stack underflow"))?;
                stack.push(a * b);
            },
            _ => panic!(),
        }
    }

    Ok(stack[0])
}
