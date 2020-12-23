use std::cmp;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let numbers: Vec<i32> = parse_numbers();
    println!("Result: {:?}", get_nth_number(&numbers, 2020));
}

pub fn part_two() {
    println!("--- Part Two ---");
    let numbers: Vec<i32> = parse_numbers();
    let result = get_nth_number(&numbers, 30_000_000);
    println!("Result: {:?}", result);
}

fn parse_numbers() -> Vec<i32> {
    return common::read_strings("./data/dec_15.txt")[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

fn get_nth_number(initial: &Vec<i32>, n: i32) -> i32 {
    let mut map: Vec<i32> = vec![0; n as usize];
    let mut state = 0 as i32;
    for i in 0..initial.len() as i32 {
        map[state as usize] = i;
        state = initial[i as usize];
    }
    for i in (initial.len() as i32)..n {
        let v = map[state as usize];
        map[state as usize] = i;
        state = (i - v) * cmp::min(v, 1);
    }
    return state;
}
