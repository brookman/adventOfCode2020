use crate::common;

struct Seat {
    // row: i32,
    // column: i32,
    id: i32,
}

impl Seat {
    fn parse(string: &String) -> Seat {
        let binary_string: String = string.chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => '0'
            })
            .collect();
        let value = i32::from_str_radix(&binary_string, 2).unwrap();
        return Seat {
            // row: value >> 3,
            // column: value & 7,
            id: value,
        };
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let result: i32 = common::read_strings("./data/dec_05.txt").iter()
        .map(|s| Seat::parse(s))
        .map(|p| p.id)
        .max()
        .unwrap_or(0);
    println!("Result: {}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut ids: Vec<i32> = common::read_strings("./data/dec_05.txt").iter()
        .map(|s| Seat::parse(s))
        .map(|s| s.id)
        .collect();

    ids.sort();

    let result = find_missing_value_in_sequence(&ids);
    println!("Result: {}", result);
}

fn find_missing_value_in_sequence(ids: &Vec<i32>) -> i32 {
    for (i, id) in ids.iter().enumerate() {
        if i + 1 < ids.len() && ids[i + 1] != id + 1 {
            return id + 1;
        }
    }
    return -1;
}
