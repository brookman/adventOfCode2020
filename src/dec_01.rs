use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;

use crate::common;

pub fn part_one() {
    let numbers = common::read_numbers("./data/dec_01.txt");
    let mut numbers_set = HashSet::new();
    for number in numbers {
        let inverse = 2020 - number;
        if numbers_set.contains(&inverse) {
            let result = number * inverse;
            println!("{}", result);
            return;
        } else {
            numbers_set.insert(number);
        }
    }
}

pub fn part_two() {

}
