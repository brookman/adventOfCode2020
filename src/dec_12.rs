use std::f32::consts::PI;
use std::ops::{Add, Mul};

use crate::common;

#[derive(Clone, Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn rot_to_vec(degrees: f64) -> Vector2 {
    let rad = degrees * PI as f64 / 180.0;
    return Vector2 { x: rad.cos().round() as i32, y: rad.sin().round() as i32 };
}

// fn vec_to_rot(vec: Vector2) -> f64 {
//     return vec.y.atan2(vec.x) / PI as f64 * 180.0;
// }

pub fn part_one() {
    println!("--- Part One ---");

    let mut pos = Vector2 { x: 0, y: 0 };
    let mut rot = 0;
    let mut lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { pos = pos + Vector2 { x: 0, y: -value } }
            'E' => { pos = pos + Vector2 { x: value, y: 0 } }
            'S' => { pos = pos + Vector2 { x: 0, y: value } }
            'W' => { pos = pos + Vector2 { x: -value, y: 0 } }
            'L' => { rot -= value }
            'R' => { rot += value }
            'F' => { pos = pos + (rot_to_vec(rot as f64) * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut pos = Vector2 { x: 0, y: 0 };
    let mut way = Vector2 { x: 10, y: -1 };
    let mut lines = common::read_strings("./data/dec_12.txt");

    for line in lines {
        let action = &line[0..1].chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            'N' => { way = way + Vector2 { x: 0, y: -value } }
            'E' => { way = way + Vector2 { x: value, y: 0 } }
            'S' => { way = way + Vector2 { x: 0, y: value } }
            'W' => { way = way + Vector2 { x: -value, y: 0 } }
            'L' => {
                match value {
                    90 => { way = Vector2 { x: way.y, y: -way.x } }
                    180 => { way = Vector2 { x: -way.x, y: -way.y } }
                    270 => { way = Vector2 { x: -way.y, y: way.x } }
                    _ => {}
                }
            }
            'R' => {
                match value {
                    90 => { way = Vector2 { x: -way.y, y: way.x } }
                    180 => { way = Vector2 { x: -way.x, y: -way.y } }
                    270 => { way = Vector2 { x: way.y, y: -way.x } }
                    _ => {}
                }
            }
            'F' => { pos = pos + (way.clone() * value) }
            _ => {}
        }
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}
