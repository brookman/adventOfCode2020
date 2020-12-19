use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

#[derive(Clone, Debug)]
struct Field {
    name: String,
    range_1: RangeInclusive<u32>,
    range_2: RangeInclusive<u32>,
}

impl Field {
    fn parse(line: &String) -> Field {
        lazy_static! {
            static ref LINE: Regex = Regex::new(r"^(.+)?: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        if LINE.is_match(&line) {
            let caps = LINE.captures(&line).unwrap();
            return Field {
                name: caps[1].to_string(),
                range_1: RangeInclusive::new(caps[2].parse::<u32>().unwrap(), caps[3].parse::<u32>().unwrap()),
                range_2: RangeInclusive::new(caps[4].parse::<u32>().unwrap(), caps[5].parse::<u32>().unwrap()),
            };
        } else {
            panic!("error: {}", line);
        }
    }

    fn is_valid(&self, number: &u32) -> bool {
        return self.range_1.contains(number) || self.range_2.contains(number);
    }
}

#[derive(Clone, Debug)]
struct Ticket {
    numbers: Vec<u32>
}

impl Ticket {
    fn parse(line: &String) -> Ticket {
        return Ticket {
            numbers: line.split(",").map(|s| s.parse::<u32>().unwrap()).collect()
        };
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let lines = common::read_strings("./data/dec_16.txt");

    let fields: Vec<Field> = (0..20).map(|i| Field::parse(&lines[i])).collect();
    let my_ticket: Ticket = Ticket::parse(&lines[22]);
    let nearby_tickets: Vec<Ticket> = (25..lines.len()).map(|i| Ticket::parse(&lines[i])).collect();

    let result: u32 = nearby_tickets.iter()
        .flat_map(|t| &t.numbers)
        .filter(|n| !is_valid(&fields, n))
        .sum();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result = 0;
    println!("Result: {:?}", result);
}

fn is_valid(fields: &Vec<Field>, number: &u32) -> bool {
    return fields.iter().any(|f| f.is_valid(number));
}
