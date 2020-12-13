use gcd::Gcd;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let lines = common::read_strings("./data/dec_13.txt");
    let time = lines[0].parse::<i32>().unwrap();
    let result = lines[1].split(",")
        .map(|b| b.to_string())
        .filter(|b| b != "x")
        .map(|b| b.parse::<i32>().unwrap())
        .map(|b| (b - (time % b), b))
        .min()
        .map(|min| min.0 * min.1)
        .unwrap();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let lines = common::read_strings("./data/dec_13.txt");
    let numbers: Vec<(i32, i32)> = lines[1].split(",")
        .map(|b| b.to_string())
        .enumerate()
        .filter(|(i, b)| b != "x")
        .map(|(i, b)| (i as i32, b.parse::<i32>().unwrap()))
        .collect();

     println!("Result: {:?}", find_time(&numbers));
}

fn find_time(numbers: &Vec<(i32, i32)>) -> u64 {
    let mut increment = 1;
    let mut result = 0;

    for (i_s, n_s) in numbers {
        let i = *i_s as u64;
        let n = *n_s as u64;
        while (result + i) % n != 0 {
            result += increment;
        }
        increment = lcm(increment, n);
    }

    return result;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a / a.gcd(b) * b;
}
