use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

#[derive(Clone, Debug)]
struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    pub fn new() -> Mask {
        return Mask {
            zeros: 0,
            ones: 0,
        };
    }

    pub fn parse(string: &String) -> Mask {
        let mut zeros: u64 = 0;
        let mut ones: u64 = 0;

        for c in string.chars() {
            zeros = zeros << 1;
            ones = ones << 1;
            match c {
                '0' => {
                    zeros |= 1;
                }
                '1' => {
                    ones |= 1;
                }
                _ => {}
            }
        }

        return Mask {
            zeros,
            ones,
        };
    }

    pub fn parse_2(string: &String) -> Vec<Mask> {
        let len = string.len();

        let mut masks: Vec<Mask> = Vec::new();

        let mut mask = Mask::parse(string);
        mask = Mask {
            zeros: 0,
            ones: mask.ones.clone(),
        };
        masks.push(mask);

        for (i, c) in string.chars().enumerate() {
            let pos = len - i - 1;
            match c {
                'X' => {
                    let mut new_masks: Vec<Mask> = Vec::new();
                    for mask in &masks {
                        new_masks.push(Mask {
                            zeros: mask.zeros.clone() | 1 << pos,
                            ones: mask.ones.clone(),
                        });
                        new_masks.push(Mask {
                            zeros: mask.zeros.clone(),
                            ones: mask.ones.clone() | 1 << pos,
                        });
                    }
                    masks.clear();
                    masks.extend(new_masks);
                }
                _ => {}
            }
        }

        return masks;
    }

    pub fn apply(&self, value: u64) -> u64 {
        return (value | self.ones) & !self.zeros;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (.{36})$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: Mask = Mask::new();

    let lines = common::read_strings("./data/dec_14.txt");

    for line in lines {
        if MASK.is_match(&line) {
            let mask_string = MASK.captures(&line).unwrap()[1].to_string();
            mask = Mask::parse(&mask_string);
        } else if MEM.is_match(&line) {
            let c = MEM.captures(&line).unwrap();
            let mem_address = (&c[1]).parse::<u64>().unwrap();
            let mem_value = (&c[2]).parse::<u64>().unwrap();
            memory.insert(mem_address, mask.apply(mem_value));
        } else {
            panic!("error: {}", line);
        }
    }

    let result: u64 = memory.iter().map(|(_, v)| *v).sum::<u64>();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (.{36})$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut masks: Vec<Mask> = vec![];

    let lines = common::read_strings("./data/dec_14.txt");

    for line in lines {
        if MASK.is_match(&line) {
            let mask_string = MASK.captures(&line).unwrap()[1].to_string();
            masks = Mask::parse_2(&mask_string);
        } else if MEM.is_match(&line) {
            let c = MEM.captures(&line).unwrap();
            let mem_address = (&c[1]).parse::<u64>().unwrap();
            let mem_value = (&c[2]).parse::<u64>().unwrap();
            for mask in &masks {
                memory.insert(mask.apply(mem_address), mem_value);
            }
        } else {
            panic!("error: {}", line);
        }
    }

    let result: u64 = memory.iter().map(|(_, v)| *v).sum::<u64>();
    println!("Result: {:?}", result);
}
