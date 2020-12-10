use std::collections::HashSet;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let numbers = common::read_larger_numbers("./data/dec_09.txt");
    let first_invalid_number = find_first_invalid_number(&numbers);
    println!("Result: {}", first_invalid_number);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let numbers = common::read_larger_numbers("./data/dec_09.txt");
    let first_invalid_number = find_first_invalid_number(&numbers);

    let mut first_index = 0;
    let mut last_index = 0;

    loop {
        let slice = &numbers[first_index..last_index];
        let sum: i64 = slice.iter().sum();
        if sum == first_invalid_number {
            println!("Result: {} ", slice.iter().min().unwrap() + slice.iter().max().unwrap());
            break;
        } else if sum < first_invalid_number {
            last_index += 1;
        } else if sum > first_invalid_number {
            first_index += 1;
        }
    }
}

fn find_first_invalid_number(numbers: &Vec<i64>) -> i64 {
    let mut window: HashSet<i64> = HashSet::new();
    let mut counter = 0;

    while counter < numbers.len() {
        if counter >= 24 {
            if !contains_sum(&window, numbers[counter] as i64) {
                return numbers[counter];
                break;
            }
        }

        if window.len() >= 25 {
            window.remove(&numbers[counter - 25]);
        }
        window.insert(numbers[counter]);
        counter += 1;
    }
    return -1;
}

fn contains_sum(numbers: &HashSet<i64>, sum: i64) -> bool {
    for number in numbers {
        if numbers.contains(&(sum - number)) {
            return true;
        }
    }
    return false;
}
