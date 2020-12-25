use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let lines = common::read_strings("./data/dec_25.txt");

    let pub1 = lines[0].parse::<u64>().unwrap();
    let pub2 =  lines[1].parse::<u64>().unwrap();

    let modulus = 20201227u64;
    let subject = 7u64;

    let mut loop_size = 0;
    let mut current = 1;

    while current != pub1 {
        loop_size += 1;
        current = (current * subject) % modulus;
    }

    current = 1;
    for _ in 0..loop_size as usize {
        current = (current * pub2) % modulus;
    }

    println!("Result: {}", current);
}
