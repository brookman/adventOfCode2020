use std::collections::HashSet;

use crate::common;

const YEAR: i32 = 2020;

pub fn part_one() {
    println!("--- Part One ---");

    let numbers = common::read_numbers("./data/dec_01.txt");
    let addends = find_addends(&numbers, YEAR);
    match addends {
        Some(a) => println!("Result: {}, {}",
                            common::format_to_sum(&vec!(a.0, a.1)),
                            common::format_to_product(&vec!(a.0, a.1))),
        None => println!("Could not find addends."),
    }
}

pub fn part_two() {
    println!("--- Part Two ---");

    let numbers = common::read_numbers("./data/dec_01.txt");
    for number in &numbers {
        let addends = find_addends(&numbers, YEAR - number);
        match addends {
            Some(s) => println!("Result: {}, {}",
                                common::format_to_sum(&vec!(*number, s.0, s.1)),
                                common::format_to_product(&vec!(*number, s.0, s.1))),
            None => {}
        }
    }
}

fn find_addends(numbers: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut numbers_set = HashSet::new();
    for number in numbers {
        let complement = sum - number;
        if numbers_set.contains(&complement) {
            return Some((*number, complement));
        } else {
            numbers_set.insert(number);
        }
    }
    return None;
}
