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

fn vec_for_rot(degrees: i32) -> Vector2 {
    let rad = degrees as f64 * PI as f64 / 180.0;
    return Vector2 { x: rad.cos().round() as i32, y: rad.sin().round() as i32 };
}

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
            'F' => { pos = pos + (vec_for_rot(rot) * value) }
            _ => {}
        }

        println!("pos: {:?}, rot: {}", pos, rot);
    }

    println!("Result: {:?}", pos.x.abs() + pos.y.abs());
}

pub fn part_two() {
    println!("--- Part One ---");


    println!("Result: {:?}", 1);
}
