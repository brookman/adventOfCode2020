use std::collections::HashSet;

use itertools::Itertools;

use crate::common;

#[derive(Clone, Debug)]
struct Grid {
    dimensions: usize,
    permutations: HashSet<Vec<i32>>,
    cells: HashSet<Vec<i32>>,
}

impl Grid {
    fn new(dimensions: usize) -> Self {
        let zero = vec![0i32; dimensions];
        let permutations = vec![-1, 0, 1].into_iter()
            .combinations_with_replacement(dimensions)
            .flat_map(|co| co.into_iter()
                .permutations(dimensions)
                .unique())
            .filter(|permutation| permutation != &zero)
            .collect();

        Self {
            dimensions,
            permutations,
            cells: HashSet::new(),
        }
    }

    fn parse(filename: &str, dimensions: usize) -> Self {
        let lines = common::read_strings(filename);
        let mut grid = Grid::new(dimensions);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let mut coords = vec![0i32; dimensions];
                    coords[0] = x as i32;
                    coords[1] = y as i32;
                    grid.cells.insert(coords);
                }
            }
        }
        grid
    }

    fn is_set(&self, coords: &[i32]) -> bool {
        self.cells.contains(coords)
    }

    fn count_neighbors(&self, coords: &[i32]) -> usize {
        self.neighbors(&coords).into_iter().filter(|n| self.is_set(n)).count()
    }

    fn neighbors(&self, tile: &[i32]) -> Vec<Vec<i32>> {
        self.permutations.iter()
            .map(|permutation| Grid::add(tile, permutation))
            .collect()
    }

    fn add(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut z: Vec<i32> = vec![0i32; a.len()];
        for ((zval, aval), bval) in z.iter_mut().zip(a).zip(b) {
            *zval = aval + bval;
        }
        z
    }

    fn count_all(&self) -> usize {
        self.cells.len()
    }

    fn iterate(&mut self) {
        self.cells = self.get_affected_tiles().into_iter()
            .filter(|tile| {
                let count = self.count_neighbors(&tile);
                self.is_set(&tile) && count == 2 || count == 3
            }).collect();
    }

    fn get_affected_tiles(&self) -> HashSet<Vec<i32>> {
        let mut result = HashSet::new();
        result.extend(self.cells.clone());
        for tile in &self.cells {
            result.extend(self.neighbors(tile));
        }
        result
    }
}

pub fn part_one() {
    println!("--- Part One ---");
    let mut grid = Grid::parse("./data/dec_17.txt", 3);
    for _ in 0..6 {
        grid.iterate();
    }
    println!("Result: {}", grid.count_all());
}

pub fn part_two() {
    println!("--- Part Two ---");
    let mut grid = Grid::parse("./data/dec_17.txt", 4);
    for _ in 0..6 {
        grid.iterate();
    }
    println!("Result: {}", grid.count_all());
}
