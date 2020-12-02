use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_numbers(filename: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let number = l.parse::<i32>().unwrap();
                numbers.push(number);
            }
        }
    }
    return numbers;
}

pub fn read_strings(filename: &str) -> Vec<String> {
    let mut strings = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                strings.push(l);
            }
        }
    }
    return strings;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
