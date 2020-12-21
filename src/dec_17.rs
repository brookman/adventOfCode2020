use std::convert::TryInto;
use std::fmt;
use std::ops::{BitAnd, BitOr};

use hilbert::Point;
use hilbert::transform::fast_hilbert::{hilbert_axes, hilbert_index};
use itertools::Itertools;
use num::{Num, NumCast, ToPrimitive};
use num::bigint::BigUint;

use crate::common;

const BITS_PER_DIMENSION: usize = 8;

fn to_coords(index: usize, dimensions: usize) -> Vec<u32> {
    return hilbert_axes(&BigUint::from(index), BITS_PER_DIMENSION, dimensions);
}

fn to_index(coords: &Vec<u32>) -> usize {
    let index = hilbert_index(coords, BITS_PER_DIMENSION, None);
    return index.to_u32().unwrap() as usize;
}

fn add(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    return a.iter().zip(b).map(|(av, bv)| av + bv).collect();
}

fn cast_to_i32(a: &Vec<u32>) -> Vec<i32> {
    return a.into_iter().map(|v| *v as i32).collect();
}

fn cast_to_u32(a: &Vec<i32>) -> Vec<u32> {
    return a.into_iter().map(|v| *v as u32).collect();
}

#[derive(Clone, Debug)]
struct Grid<const D: usize> {
    size: usize,
    cells: Vec<u8>,
}

impl<const D: usize> Grid<{ D }> {
    fn new(size: usize) -> Grid<{ D }> {
        return Grid {
            size,
            cells: vec![0; size.pow(D as u32) * BITS_PER_DIMENSION as usize],
        };
    }

    fn parse(filename: &str) -> Grid<{ D }> {
        let lines = common::read_strings(filename);
        let mut state = Grid::new(lines.len());
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut coords = vec![(lines.len() / 2) as u32; D];
                coords[0] = x as u32;
                coords[1] = y as u32;
                state.set(&coords, if c == '#' { 1 } else { 0 });
            }
        }
        return state;
    }

    fn set(&mut self, coords: &Vec<u32>, value: u8) {
        self.set_index(to_index(coords), value);
    }

    fn set_index(&mut self, index: usize, value: u8) {
        self.cells[index] = value;
    }

    fn get(&self, coords: &Vec<i32>) -> u8 {
        for c in coords.iter() {
            if *c < 0 || *c > self.size as i32 {
                return 0;
            }
        }
        return self.cells[to_index(&cast_to_u32(&coords))];
    }

    fn count_all(&self) -> u32 {
        return self.cells.iter().map(|c| *c as u32).sum();
    }

    fn count_neighbors(&self, vec_coords: &Vec<i32>) -> u32 {
        let mut sum = 0u32;

        for permutation in vec![-1, 0, 1].into_iter().combinations_with_replacement(D)
            .flat_map(|co| co.into_iter().permutations(D).unique()) {
            if permutation == vec![0, 0, 0] {
                continue;
            }

            sum += self.get(&add(&vec_coords, &permutation)) as u32;
        }

        return sum;
    }

    fn next(&self) -> Grid<{ D }> {
        let mut new_grid = Grid::new(self.size + 2);
        for cell in 0..new_grid.cells.len() {
            let coords = to_coords(cell, D);
            let old_coord = add(&cast_to_i32(&coords), &vec![-1; coords.len()]);
            let old_value = self.get(&old_coord);
            let count = self.count_neighbors(&old_coord);
            let mut value = 0;
            if old_value > 0 {
                if count == 2 || count == 3 {
                    value = 1;
                }
            } else {
                if count == 3 {
                    value = 1;
                }
            }
            new_grid.set_index(cell, value);
        }
        return new_grid;
    }
}

impl<const D: usize> fmt::Display for Grid<{ D }> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nsize={}, dimensions={}, cells={}\n", self.size, D, self.cells.len());
        for z in 0..self.size {
            write!(f, "z={}\n", z);
            for y in 0..self.size {
                for x in 0..self.size {
                    if self.get(&vec![x as i32, y as i32, z as i32]) > 0 {
                        write!(f, "#");
                    } else {
                        write!(f, ".");
                    }
                }
                write!(f, "\n");
            }
        }
        write!(f, "")
    }
}

pub fn part_one() {
    println!("--- Part One ---");
    let mut state = Grid::<3>::parse("./data/dec_17.txt");
    for _ in 0..6 {
        state = state.next();
    }
    println!("Result: {}", state.count_all());
}

pub fn part_two() {
    println!("--- Part One ---");
    let mut state = Grid::<4>::parse("./data/dec_17.txt");
    for _ in 0..6 {
        state = state.next();
    }
    println!("Result: {}", state.count_all());
}
