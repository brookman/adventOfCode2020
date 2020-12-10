use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

#[derive(Clone, Debug)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

pub fn part_one() {
    println!("--- Part One ---");

    let instructions = parse("./data/dec_08.txt");
    let (_, acc) = run(&instructions);

    println!("Result: {}", acc);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let instructions = parse("./data/dec_08.txt");

    let mut instruction_to_flip = 0;
    let mut success = false;
    let mut acc = 0;
    while !success {
        let mut instructions_copy = instructions.clone();
        let mut instruction = instructions_copy[instruction_to_flip].clone();
        match instruction.operation {
            Operation::ACC => {}
            Operation::JMP => {
                instruction = Instruction {
                    operation: Operation::NOP,
                    argument: instruction.argument,
                }
            }
            Operation::NOP => {
                instruction = Instruction {
                    operation: Operation::JMP,
                    argument: instruction.argument,
                }
            }
        }
        instructions_copy[instruction_to_flip] = instruction.clone();
        instruction_to_flip += 1;

        let res = run(&instructions_copy);
        success = res.0;
        acc = res.1;
    }

    println!("Result: {}", acc);
}


fn parse(filename: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref LINE: Regex = Regex::new(r"^([a-z]{3}) ([+,\-]\d+)$").unwrap();
    }

    return common::read_strings(filename).iter()
        .map(|line| {
            if LINE.is_match(&line) {
                let caps = LINE.captures(&line).unwrap();
                return Instruction {
                    operation: match &caps[1] {
                        "acc" => Operation::ACC,
                        "jmp" => Operation::JMP,
                        _ => Operation::NOP,
                    },
                    argument: caps[2].parse::<i32>().unwrap(),
                };
            } else {
                panic!("error: {}", line);
            }
        }).collect();
}

fn run(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut ip: i32 = 0;
    let mut processed_instructions: HashSet<i32> = HashSet::new();

    loop {
        if processed_instructions.contains(&ip) {
            return (false, acc);
        }
        if ip >= instructions.len() as i32 {
            return (true, acc);
        }
        processed_instructions.insert(ip);

        let instruction = &instructions[ip as usize];
        match instruction.operation {
            Operation::ACC => {
                acc += instruction.argument;
                ip += 1;
            }
            Operation::JMP => {
                ip += instruction.argument;
            }
            Operation::NOP => {
                ip += 1;
            }
        }
    }
}
