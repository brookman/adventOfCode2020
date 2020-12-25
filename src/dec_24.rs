extern crate nom;

use std::collections::{HashSet};

use crate::common;
use crate::vectors::Vector3i;

use self::nom::{IResult};
use self::nom::branch::alt;
use self::nom::bytes::complete::tag;
use self::nom::multi::many0;

fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}

pub fn part_one() {
    println!("--- Part One ---");
    let lines = common::read_strings("./data/dec_24.txt");
    let mut tiles: HashSet<Vector3i> = HashSet::new();

    for line in lines {
        let res: IResult<&str, Vec<&str>> = many0(
            alt((
                tag("e"),
                tag("se"),
                tag("sw"),
                tag("w"),
                tag("nw"),
                tag("ne"),
            )))(line.as_str());
        if let Ok(r) = res {
            let vecs: Vec<Vector3i> = r.1.into_iter().map(|s|
                match s {
                    "e" => Vector3i::new(1, -1, 0),
                    "se" => Vector3i::new(0, -1, 1),
                    "sw" => Vector3i::new(-1, 0, 1),
                    "w" => Vector3i::new(-1, 1, 0),
                    "nw" => Vector3i::new(0, 1, -1),
                    "ne" => Vector3i::new(1, 0, -1),
                    _ => Vector3i::new(0, 0, 0),
                }
            ).collect();
            let mut sum = Vector3i::new(0, 0, 0);
            for v in vecs {
                sum = sum + v;
            }
            if tiles.contains(&sum) {
                tiles.remove(&sum);
            } else {
                tiles.insert(sum);
            }
        }
    }

    println!("Result: {}", tiles.len());
}

pub fn part_two() {
    println!("--- Part One ---");

    println!("Result: {:?}", 0);
}
