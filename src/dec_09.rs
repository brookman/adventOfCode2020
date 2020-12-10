use std::collections::HashSet;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let numbers = common::read_larger_numbers("./data/dec_09.txt");

    let mut window: HashSet<i64> = HashSet::new();
    let mut counter = 0;

    while counter < numbers.len() {
        if counter >= 24 {
            if !contains_sum(&window, numbers[counter] as i64) {
                break;
            }
        }

        if window.len() >= 25 {
            window.remove(&numbers[counter - 25]);
        }
        window.insert(numbers[counter]);
        counter += 1;
    }
}

pub fn part_two() {
    println!("--- Part Two ---");

    let lines = common::read_strings("./data/dec_09.txt");
    println!("Result: {}", 0);
}

fn contains_sum(numbers: &HashSet<i64>, sum: i64) -> bool {
    for number in numbers {
        if numbers.contains(&(sum - number)) {
            return true;
        }
    }
    return false;
}
