use std::fs::File;
use std::io::{BufRead, BufReader, Lines, LineWriter, Result, Write};
use std::path::Path;
use std::str::FromStr;

use regex::{Regex};

pub trait Re {
    fn re<T: FromStr>(&self, re: &str, index: usize) -> T;
}

impl Re for String {
    fn re<T: FromStr>(&self, re: &str, index: usize) -> T {
        let regex = Regex::new(re).unwrap();
        return match regex.captures(self) {
            Some(caps) => {
                match caps[index].parse::<T>() {
                    Ok(value) => Some(value),
                    _ => None
                }
            }
            None => { None }
        }.unwrap();
    }
}

pub fn read_numbers(filename: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let number = line.parse::<i32>().unwrap();
            numbers.push(number);
        }
    }
    numbers
}

pub fn read_larger_numbers(filename: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let number = line.parse::<i64>().unwrap();
            numbers.push(number);
        }
    }
    numbers
}

pub fn read_strings(filename: &str) -> Vec<String> {
    let mut strings = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            strings.push(line);
        }
    }
    strings
}


pub fn write_strings(filename: &str, lines: &Vec<String>) -> Result<()> {
    let file = File::create(filename)?;
    let mut file = LineWriter::new(file);

    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    file.flush()?;

    Ok(())
}

// read chunks of lines separated by empty lines
pub fn read_chunks(filename: &str) -> Vec<Vec<String>> {
    let mut chunks = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mut strings = Vec::new();
        for line in lines.flatten() {
            if line.trim().is_empty() {
                chunks.push(strings);
                strings = Vec::new();
            } else {
                strings.push(line);
            }
        }
        if !strings.is_empty() {
            chunks.push(strings);
        }
    }
    chunks
}

pub fn format_to_sum(numbers: &[i32]) -> String {
    let strings: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
    let joined = strings.join(" + ");
    let sum: i32 = numbers.iter().sum();
    format!("{} = {}", joined, sum)
}

pub fn format_to_product(numbers: &[i32]) -> String {
    let strings: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
    let joined = strings.join(" * ");
    let product: i64 = numbers.iter().map(|n| (*n as i64)).product();
    format!("{} = {}", joined, product)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
