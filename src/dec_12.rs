use crate::common;
use crate::vectors::Vector2i;

pub fn part_one() {
    println!("--- Part One ---");

    let mut pos = Vector2i { x: 0, y: 0 };
    let mut rot = 0;
    let lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { pos = pos + Vector2i { x: 0, y: -value } }
            'E' => { pos = pos + Vector2i { x: value, y: 0 } }
            'S' => { pos = pos + Vector2i { x: 0, y: value } }
            'W' => { pos = pos + Vector2i { x: -value, y: 0 } }
            'L' => { rot -= value }
            'R' => { rot += value }
            'F' => { pos = pos + (Vector2i::from_rot(rot as f32) * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut pos = Vector2i { x: 0, y: 0 };
    let mut way = Vector2i { x: 10, y: -1 };
    let lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { way = way + Vector2i { x: 0, y: -value } }
            'E' => { way = way + Vector2i { x: value, y: 0 } }
            'S' => { way = way + Vector2i { x: 0, y: value } }
            'W' => { way = way + Vector2i { x: -value, y: 0 } }
            'L' => {
                match value {
                    90 => { way = Vector2i { x: way.y, y: -way.x } }
                    180 => { way = Vector2i { x: -way.x, y: -way.y } }
                    270 => { way = Vector2i { x: -way.y, y: way.x } }
                    _ => {}
                }
            }
            'R' => {
                match value {
                    90 => { way = Vector2i { x: -way.y, y: way.x } }
                    180 => { way = Vector2i { x: -way.x, y: -way.y } }
                    270 => { way = Vector2i { x: way.y, y: -way.x } }
                    _ => {}
                }
            }
            'F' => { pos = pos + (way.clone() * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}
