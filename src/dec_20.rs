use itertools::Itertools;
use ndarray::{Array, Array2, ArrayView, ArrayView2, Axis, concatenate, Ix1, s};
use num::integer::Roots;

use crate::common;
use crate::common::Re;

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    data: Array2<bool>,
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    tiles: Vec<Option<usize>>,
}

impl Tile {
    pub fn parse(filename: &str) -> (usize, usize, Vec<Tile>) {
        let mut result = Vec::new();
        let mut tile_count = 0;
        let mut sile_size = 0;
        for chunk in &common::read_strings(filename).iter().chunks(12) {
            let block: Vec<String> = chunk.map(|line| line.trim().to_string()).filter(|line| !line.is_empty()).collect();
            let id = block[0].re::<u32>(r"^Tile (\d+):$", 1);
            sile_size = block[1].len();
            let bits: Vec<bool> = block[1..].join("").chars().map(|c| c == '#').collect();

            let data: Array2<bool> = Array::from_shape_vec((sile_size, sile_size), bits).unwrap();
            let main = Tile { id, data: data.clone() };
            for variant in main.get_variants() {
                result.push(variant);
            }
            tile_count += 1;
        }
        (tile_count, sile_size, result)
    }

    pub fn get_variants(&self) -> Vec<Tile> {
        let mirror = self.data.clone().slice_move(s![..,..;-1]);
        return vec![
            Tile { id: self.id, data: self.data.clone() },
            Tile { id: self.id, data: self.data.clone().t().to_owned().slice_move(s![..,..;-1]) }, // rot 90
            Tile { id: self.id, data: self.data.clone().slice_move(s![..;-1,..;-1]) }, // rot 180
            Tile { id: self.id, data: self.data.clone().t().to_owned().slice_move(s![..;-1,..]) }, // rot 270
            Tile { id: self.id, data: mirror.clone() },
            Tile { id: self.id, data: mirror.t().to_owned().slice_move(s![..,..;-1]) }, // rot 90
            Tile { id: self.id, data: mirror.clone().slice_move(s![..;-1,..;-1]) }, // rot 180
            Tile { id: self.id, data: mirror.t().to_owned().slice_move(s![..;-1,..]) } // rot 270
        ];
    }

    pub fn top(&self) -> ArrayView<bool, Ix1> {
        self.data.slice(s![0,..])
    }

    pub fn bottom(&self) -> ArrayView<bool, Ix1> {
        self.data.slice(s![-1,..])
    }

    pub fn left(&self) -> ArrayView<bool, Ix1> {
        self.data.slice(s![..,0])
    }

    pub fn right(&self) -> ArrayView<bool, Ix1> {
        self.data.slice(s![..,-1])
    }

    pub fn crop(&self) -> Tile {
        Tile {
            id: self.id,
            data: self.data.clone().slice_move(s![1..-1,1..-1]),
        }
    }
}

impl Grid {
    fn arrange_tiles(&mut self, all_tiles: &[Tile], put_index: usize) -> bool {
        for i in 0..all_tiles.len() {
            if self.fits_at(all_tiles, i, put_index) {
                self.tiles[put_index] = Some(i);
                if put_index + 1 < self.tiles.len() {
                    if self.arrange_tiles(all_tiles, put_index + 1) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn fits_at(&self, all_tiles: &[Tile], tile_index: usize, put_index: usize) -> bool {
        let tile = &all_tiles[tile_index];
        for i in 0..put_index {
            let index = self.tiles[i];
            let compare_tile = &all_tiles[index.unwrap()];
            if compare_tile.id == tile.id {
                return false;
            }
        }

        let x = put_index % self.size;
        let y = put_index / self.size;

        if x > 0 {
            let index = self.tiles[put_index - 1];
            let left_tile = &all_tiles[index.unwrap()];
            if tile.left() != left_tile.right() {
                return false;
            }
        }

        if y > 0 {
            let index = self.tiles[put_index - self.size];
            let top_tile = &all_tiles[index.unwrap()];
            if tile.top() != top_tile.bottom() {
                return false;
            }
        }
        true
    }

    fn get_tile_id(&self, all_tiles: &[Tile], tile_index: usize) -> u64 {
        all_tiles[self.tiles[tile_index].unwrap()].id as u64
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let (tile_count, _, tiles) = Tile::parse("./data/dec_20.txt");
    let grid_size = tile_count.sqrt();
    let mut grid = Grid {
        size: grid_size,
        tiles: vec![None; grid_size * grid_size],
    };
    grid.arrange_tiles(&tiles, 0);

    let corner1 = grid.get_tile_id(&tiles, 0);
    let corner2 = grid.get_tile_id(&tiles, grid_size - 1);
    let corner3 = grid.get_tile_id(&tiles, grid_size * grid_size - 1);
    let corner4 = grid.get_tile_id(&tiles, grid_size * grid_size - grid_size);

    let result = corner1 * corner2 * corner3 * corner4;
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let (tile_count, _, tiles) = Tile::parse("./data/dec_20.txt");

    let grid_size = tile_count.sqrt();
    let mut grid = Grid {
        size: grid_size,
        tiles: vec![None; grid_size * grid_size],
    };
    grid.arrange_tiles(&tiles, 0);

    let rows = (0..grid_size).map(|row_index| {
        let tiles = (0..grid_size)
            .map(|col| grid.tiles[row_index * grid_size + col].unwrap())
            .map(|id| &tiles[id])
            .map(|t| t.crop())
            .collect::<Vec<Tile>>();

        let row = tiles.iter()
            .map(|tile| tile.data.view())
            .collect::<Vec<ArrayView2<bool>>>();

        concatenate(Axis(1), &row).unwrap()
    }).collect::<Vec<Array2<bool>>>();

    let image = Tile {
        id: 0,
        data: concatenate(Axis(0),
                          &rows.iter()
                              .map(|row| row.view())
                              .collect::<Vec<ArrayView2<bool>>>(),
        ).unwrap(),
    };

    let shape = (3, 20);
    let sea_monster_vec: Vec<bool> = "                  # #    ##    ##    ### #  #  #  #  #  #   ".chars()
        .map(|c| c == '#')
        .collect();
    let sea_monster: Array2<bool> = Array::from_shape_vec(shape, sea_monster_vec).unwrap();

    let mut times_found = 0;
    for variant in image.get_variants() {
        for window in variant.data.windows(shape) {
            if window.clone().to_owned() & &sea_monster == sea_monster {
                times_found += 1;
            }
        }
        if times_found > 0 {
            break;
        }
    }

    let cnt = image.data.iter().filter(|v| **v).count();
    println!("Result: {:?}", cnt - (times_found * 15));
}


