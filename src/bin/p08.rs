use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Debug, Default)]
struct CPU {
    ip: i64,
    acc: i64,
}

impl CPU {
    fn new() -> Self {
        Default::default()
    }

    fn step(&mut self, instructions: &Vec<Instruction>) {
        match instructions[self.ip as usize] {
            Instruction::Acc(x) => self.acc += x,
            Instruction::Jmp(x) => self.ip += x - 1,
            Instruction::Nop(_) => { },
        }
        self.ip += 1;
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }
}

fn main() -> Result<()> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in io::stdin().lock().lines() {
        let op_arg: Vec<String> = line?.split(" ").map(|x| x.to_owned()).collect();
        let arg = op_arg[1].parse()?;
        let i = match &*op_arg[0] {
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            "nop" => Instruction::Nop(arg),
            _ => panic!(),
        };
        instructions.push(i);
    }

    let max_ip = instructions.len() - 1;
    let mut cpu = CPU::new();
    let mut ips = HashSet::new();

    loop {
        if !ips.insert(cpu.ip) {
            println!("Part 1 = {}", cpu.acc);
            break;
        }
        cpu.step(&instructions);
    }

    for ip in 0..max_ip {
        cpu.reset();
        ips.clear();
        flip(&mut instructions[ip as usize]);
        loop {
            if cpu.ip > max_ip as i64 {
                println!("Part 2 = {}", cpu.acc);
                return Ok(());
            }
            if !ips.insert(cpu.ip) {
                break;
            }
            cpu.step(&instructions);
        }
        flip(&mut instructions[ip as usize]);
    }

    Ok(())
}

fn flip(i: &mut Instruction) {
    *i = match i {
        Instruction::Nop(x) => Instruction::Jmp(*x),
        Instruction::Jmp(x) => Instruction::Nop(*x),
        Instruction::Acc(x) => Instruction::Acc(*x),
    };
}
