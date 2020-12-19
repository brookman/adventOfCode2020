use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;
use crate::common::format_to_product;

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

    fn all_valid(fields: &Vec<Field>, number: &u32) -> bool {
        return fields.iter().any(|f| f.is_valid(number));
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

    fn is_valid(&self, fields: &Vec<Field>) -> bool {
        return self.numbers.iter().all(|n| fields.iter().any(|f| f.is_valid(n)));
    }
}

#[derive(Clone, Debug)]
struct Data {
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
    valid_nearby_tickets: Vec<Ticket>,
    fields: Vec<Field>,
}

impl Data {
    fn parse(filename: &str) -> Data {
        const NUMBER_OF_FIELDS: usize = 20;
        const MY_TICKET_LINE: usize = 22;
        const NEARBY_TICKETS_LINE: usize = 25;

        let lines = common::read_strings(filename);

        let fields: Vec<Field> = (0..NUMBER_OF_FIELDS).map(|i| Field::parse(&lines[i])).collect();
        let my_ticket: Ticket = Ticket::parse(&lines[MY_TICKET_LINE]);
        let nearby_tickets: Vec<Ticket> = (NEARBY_TICKETS_LINE..lines.len()).map(|i| Ticket::parse(&lines[i])).collect();

        let valid_nearby_tickets: Vec<Ticket> = nearby_tickets.clone().into_iter()
            .filter(|t| t.is_valid(&fields))
            .collect();

        return Data {
            my_ticket: my_ticket.clone(),
            nearby_tickets,
            valid_nearby_tickets,
            fields: fields.clone(),
        };
    }

    fn determine_all(&self) -> HashMap<usize, usize> {
        let mut field_indices: HashSet<usize> = HashSet::new();
        let mut ticket_field_indices: HashSet<usize> = HashSet::new();
        let mut map: HashMap<usize, usize> = HashMap::new();

        for i in 0..self.fields.len() {
            field_indices.insert(i);
            ticket_field_indices.insert(i);
        }

        while field_indices.len() > 0 && ticket_field_indices.len() > 0 {
            let mut ticket_field_indices_to_remove = Vec::new();
            for ticket_field_index in &ticket_field_indices {
                let field_index = self.determine_single_field(&field_indices, *ticket_field_index);
                if field_index.is_some() {
                    map.insert(*ticket_field_index, field_index.unwrap());
                    ticket_field_indices_to_remove.push(ticket_field_index.clone());
                    field_indices.remove(&field_index.unwrap());
                }
            }
            for to_remove in ticket_field_indices_to_remove {
                ticket_field_indices.remove(&to_remove);
            }
        }
        return map;
    }

    fn determine_single_field(&self, field_indices: &HashSet<usize>, ticket_field_index: usize) -> Option<usize> {
        let numbers: Vec<u32> = self.valid_nearby_tickets.iter()
            .map(|t| t.numbers[ticket_field_index])
            .collect();

        let mut indices: Vec<usize> = Vec::new();
        for i in field_indices {
            let field = self.fields.get(*i).unwrap();
            let matches = numbers.iter()
                .all(|n| field.is_valid(n));
            if matches {
                indices.push(*i);
            }
        }
        if indices.len() == 1 {
            return Some(indices[0]);
        }
        return None;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let data = Data::parse("./data/dec_16.txt");

    let result: u32 = data.nearby_tickets.iter()
        .flat_map(|t| &t.numbers)
        .filter(|n| !Field::all_valid(&data.fields, n))
        .sum();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let data = Data::parse("./data/dec_16.txt");

    let mut map: HashMap<usize, usize> = data.determine_all();
    let p: Vec<i32> = data.my_ticket.numbers.iter().enumerate()
        .filter(|(i, n)| (&data.fields[map[&i]]).name.starts_with("departure"))
        .map(|(i, n)| *n as i32)
        .collect();
    println!("Result: {}", format_to_product(&p));
}
