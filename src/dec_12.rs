use std::f32::consts::PI;

use crate::common;
use crate::vectors::Vec2;

pub fn from_rot(degrees: f32) -> Vec2<i32> {
    let rad = degrees * PI / 180.0f32;
    Vec2::new(rad.cos().round() as i32, rad.sin().round() as i32)
}

pub fn part_one() {
    println!("--- Part One ---");

    let mut pos = Vec2::new(0, 0);
    let mut rot = 0;
    let lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().next().unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { pos = pos + Vec2::new(0, -value) }
            'E' => { pos = pos + Vec2::new(value, 0) }
            'S' => { pos = pos + Vec2::new(0, value) }
            'W' => { pos = pos + Vec2::new(-value, 0) }
            'L' => { rot -= value }
            'R' => { rot += value }
            'F' => { pos = pos + (from_rot(rot as f32) * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut pos = Vec2::new(0, 0);
    let mut way = Vec2::new(10, -1);
    let lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { way = way + Vec2::new(0, -value) }
            'E' => { way = way + Vec2::new(value, 0) }
            'S' => { way = way + Vec2::new(0, value) }
            'W' => { way = way + Vec2::new(-value, 0) }
            'L' => {
                match value {
                    90 => { way = Vec2::new(way.y, -way.x) }
                    180 => { way = Vec2::new(-way.x, -way.y) }
                    270 => { way = Vec2::new(-way.y, way.x) }
                    _ => {}
                }
            }
            'R' => {
                match value {
                    90 => { way = Vec2::new(-way.y, way.x) }
                    180 => { way = Vec2::new(-way.x, -way.y) }
                    270 => { way = Vec2::new(way.y, -way.x) }
                    _ => {}
                }
            }
            'F' => { pos = pos + (way.clone() * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}
