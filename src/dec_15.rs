use std::collections::HashMap;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let numbers: Vec<i32> = parse_numbers();
    println!("Result: {:?}", get_nth_number(&numbers, 2020));
}

pub fn part_two() {
    println!("--- Part Two ---");

    let numbers: Vec<i32> = parse_numbers();
    println!("Result: {:?}", get_nth_number(&numbers, 30_000_000));
}

fn parse_numbers() -> Vec<i32> {
    return common::read_strings("./data/dec_15.txt")[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

fn get_nth_number(starting_numbers: &Vec<i32>, n: i32) -> i32 {
    let mut map: Vec<i32> = vec![-1; n as usize];

    let mut last = *starting_numbers.last().unwrap();
    let mut count = 0;

    for n in starting_numbers {
        map[*n as usize] = count;
        last = *n;
        count += 1;
    }

    let max = n - starting_numbers.len() as i32;
    for _ in 0..max {
        let mut number = 0;
        let val = map[last as usize];
        if val > -1 {
            number = count - val - 1;
        }

        map[last as usize] = count - 1;
        last = number;
        count += 1;
    }

    return last;
}
