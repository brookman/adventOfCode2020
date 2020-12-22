use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;
use crate::common::Re;

const TILE_SIZE: usize = 10;
const GRID_SIZE: usize = 12;

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    top: Vec<bool>,
    right: Vec<bool>,
    bottom: Vec<bool>,
    left: Vec<bool>,
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    tiles: Vec<Option<usize>>,
}

struct Image {
    pixels: Vec<bool>
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Tile {
    pub fn parse(filename: &str) -> Vec<Tile> {
        return common::read_strings(filename)
            .chunks(TILE_SIZE + 2)
            .map(|chunk| {
                Tile::parse_single(&chunk.into_iter()
                    .filter(|l| !l.is_empty())
                    .map(|l| l.clone())
                    .collect())
            })
            .flat_map(|s| s.get_permutations().into_iter())
            .collect();
    }

    pub fn parse_single(lines: &Vec<String>) -> Tile {
        let id = lines[0].re::<u32>(r"^Tile (\d+):$", 1);
        let mut top = vec![];
        let mut right = vec![];
        let mut bottom = vec![];
        let mut left = vec![];
        for i in 1..lines.len() {
            let chars: Vec<bool> = lines[i].chars()
                .map(|c| { c == '#' })
                .collect();
            if i == 1 {
                top = chars.clone();
            }
            if i == lines.len() - 1 {
                bottom = chars.clone();
            }
            left.push(*chars.first().unwrap());
            right.push(*chars.last().unwrap());
        }
        return Tile {
            id,
            top,
            right,
            bottom,
            left,
        };
    }

    fn get_permutations(&self) -> Vec<Tile> {
        let mut perm = vec![];

        let original_rot0 = self.clone();
        let original_rot1 = Tile::rot(&original_rot0);
        let original_rot2 = Tile::rot(&original_rot1);
        let original_rot3 = Tile::rot(&original_rot2);
        perm.push(original_rot0);
        perm.push(original_rot1);
        perm.push(original_rot2);
        perm.push(original_rot3);

        let flipped_rot0 = Tile::flip(self);
        let flipped_rot1 = Tile::rot(&flipped_rot0);
        let flipped_rot2 = Tile::rot(&flipped_rot1);
        let flipped_rot3 = Tile::rot(&flipped_rot2);
        perm.push(flipped_rot0);
        perm.push(flipped_rot1);
        perm.push(flipped_rot2);
        perm.push(flipped_rot3);

        return perm;
    }

    fn rot(tile: &Tile) -> Tile {
        return Tile {
            id: tile.id,
            top: tile.left.clone().into_iter().rev().collect(),
            right: tile.top.clone(),
            bottom: tile.right.clone().into_iter().rev().collect(),
            left: tile.bottom.clone(),
        };
    }

    fn flip(tile: &Tile) -> Tile {
        return Tile {
            id: tile.id,
            top: tile.bottom.clone(),
            right: tile.right.clone().into_iter().rev().collect(),
            bottom: tile.top.clone(),
            left: tile.left.clone().into_iter().rev().collect(),
        };
    }
}

impl Grid {
    fn fits_at(&self, all_tiles: &Vec<Tile>, tile_index: usize, put_index: usize) -> bool {
        let tile = &all_tiles[tile_index];
        for i in 0..put_index {
            let index = self.tiles[i];
            let compare_tile = &all_tiles[index.unwrap()];
            if compare_tile.id == tile.id {
                return false;
            }
        }

        let x = put_index % GRID_SIZE;
        let y = put_index / GRID_SIZE;

        if x > 0 {
            let index = self.tiles[put_index - 1];
            let left_tile = &all_tiles[index.unwrap()];
            if !tile.left.eq(&left_tile.right) {
                return false;
            }
        }

        if y > 0 {
            let index = self.tiles[put_index - GRID_SIZE];
            let top_tile = &all_tiles[index.unwrap()];
            if !tile.top.eq(&top_tile.bottom) {
                return false;
            }
        }

        return true;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let tiles = Tile::parse("./data/dec_20.txt");
    let mut grid = Grid {
        size: GRID_SIZE,
        tiles: vec![None; GRID_SIZE * GRID_SIZE],
    };
    run(&tiles, &mut grid, 0);
    println!("grid: {:?}", grid);

    let corner1 = &tiles[grid.tiles[0].unwrap()];
    let corner2 = &tiles[grid.tiles[GRID_SIZE - 1].unwrap()];
    let corner3 = &tiles[grid.tiles[GRID_SIZE * GRID_SIZE - 1].unwrap()];
    let corner4 = &tiles[grid.tiles[GRID_SIZE * GRID_SIZE - GRID_SIZE].unwrap()];
    println!("corners: {:?} {:?} {:?} {:?}", corner1.id, corner2.id, corner3.id, corner4.id);

    let result = *(&tiles[grid.tiles[0].unwrap()].id) as u64
        * *(&tiles[grid.tiles[GRID_SIZE - 1].unwrap()].id) as u64
        * *(&tiles[grid.tiles[GRID_SIZE * GRID_SIZE - 1].unwrap()].id) as u64
        * *(&tiles[grid.tiles[GRID_SIZE * GRID_SIZE - GRID_SIZE].unwrap()].id) as u64;
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let tiles = Tile::parse("./data/dec_20.txt");
    let mut grid = Grid {
        size: GRID_SIZE,
        tiles: vec![None; GRID_SIZE * GRID_SIZE],
    };
    run(&tiles, &mut grid, 0);
    println!("grid: {:?}", grid);

    let result = 0;
    println!("Result: {:?}", result);
}

fn run(all_tiles: &Vec<Tile>, grid: &mut Grid, put_index: usize) -> bool {
    for i in 0..all_tiles.len() {
        if grid.fits_at(all_tiles, i, put_index) {
            grid.tiles[put_index] = Some(i);
            if put_index + 1 < grid.tiles.len() {
                if run(all_tiles, grid, put_index + 1) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    return false;
}
