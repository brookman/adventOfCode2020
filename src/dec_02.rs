use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

struct PasswordEntry {
    number_1: i32,
    number_2: i32,
    character: char,
    password: String,
}

impl PasswordEntry {
    fn parse(string: &str) -> PasswordEntry {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        }

        assert!(RE.is_match(string));
        let captures = RE.captures(string);
        assert!(captures.is_some());
        let c = captures.unwrap();
        assert_eq!(c.len(), 5);
        return PasswordEntry {
            number_1: (&c[1]).parse::<i32>().unwrap(),
            number_2: (&c[2]).parse::<i32>().unwrap(),
            character: (&c[3]).parse::<char>().unwrap(),
            password: String::from(&c[4]),
        };
    }

    fn is_valid_part_one(&self) -> bool {
        let count = self.password.chars().into_iter()
            .filter(|c| *c == self.character)
            .count() as i32;
        return count >= self.number_1 && count <= self.number_2;
    }

    fn is_valid_part_two(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        let first_match = chars[(self.number_1 - 1) as usize] == self.character;
        let second_match = chars[(self.number_2 - 1) as usize] == self.character;
        return first_match ^ second_match;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let count: usize = common::read_strings("./data/dec_02.txt").iter()
        .map(|s| PasswordEntry::parse(s))
        .filter(|p| p.is_valid_part_one())
        .count();
    println!("Result: {}", count)
}

pub fn part_two() {
    println!("--- Part Two ---");

    let count: usize = common::read_strings("./data/dec_02.txt").iter()
        .map(|s| PasswordEntry::parse(s))
        .filter(|p| p.is_valid_part_two())
        .count();
    println!("Result: {}", count)
}
