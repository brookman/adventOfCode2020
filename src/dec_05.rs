use crate::common;

struct Seat {
    row: i32,
    column: i32,
}

impl Seat {
    fn parse(string: &String) -> Seat {
        let digits: Vec<i32> = string.chars().into_iter()
            .map(|c| match c {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => 0
            })
            .collect();

        return Seat {
            row: digits[0] * 64 + digits[1] * 32 + digits[2] * 16 + digits[3] * 8 + digits[4] * 4 + digits[5] * 2 + digits[6],
            column: digits[7] * 4 + digits[8] * 2 + digits[9],
        };
    }

    fn get_id(&self) -> i32 {
        return self.row * 8 + self.column;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let result: i32 = common::read_strings("./data/dec_05.txt").iter()
        .map(|s| Seat::parse(s))
        .map(|p| p.get_id())
        .max()
        .unwrap_or(0);
    println!("Result: {}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");
}
