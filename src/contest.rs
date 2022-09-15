use crate::common;

struct Data {
    id: i32,
}

impl Data {
    fn parse(string: &String) -> Data {
        let binary_string: String = string.chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => '0'
            })
            .collect();
        let id = i32::from_str_radix(&binary_string, 2).unwrap();
        Data {
            id
        }
    }
}

pub fn run() {
    let result: i32 = common::read_strings("./data/dec_05.txt").iter()
        .map(Data::parse)
        .map(|p| p.id)
        .max()
        .unwrap_or(0);
    println!("Result: {}", result);
}