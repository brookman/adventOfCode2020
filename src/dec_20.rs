use itertools::Itertools;
use ndarray::{Array, Array2, ArrayView, Ix1, s};
use num::integer::Roots;

use crate::common;
use crate::common::Re;

#[derive(Debug, Clone)]
struct TileVersion {
    id: u32,
    data: Array2<char>,
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    tiles: Vec<Option<usize>>,
}

impl TileVersion {
    pub fn parse(filename: &str) -> (usize, Vec<TileVersion>) {
        let mut result = Vec::new();
        let mut chunk_count = 0;
        for chunk in &common::read_strings(filename).iter().chunks(12) {
            let block: Vec<String> = chunk.map(|line| line.trim().to_string()).filter(|line| !line.is_empty()).collect();
            let id = block[0].re::<u32>(r"^Tile (\d+):$", 1);
            let shape = (block.len() - 1, block[1].len());
            let chars: Vec<char> = block[1..].join("").chars().collect();

            let data: Array2<char> = Array::from_shape_vec(shape, chars).unwrap();
            let mirror = data.clone().slice_move(s![..,..;-1]);
            result.push(TileVersion { id, data: data.clone() });
            result.push(TileVersion { id, data: data.clone().t().to_owned().slice_move(s![..,..;-1]) }); // rot 90
            result.push(TileVersion { id, data: data.clone().slice_move(s![..;-1,..;-1]) }); // rot 180
            result.push(TileVersion { id, data: data.clone().t().to_owned().slice_move(s![..;-1,..]) }); // rot 270
            result.push(TileVersion { id, data: mirror.clone() });
            result.push(TileVersion { id, data: mirror.clone().t().to_owned().slice_move(s![..,..;-1]) });// rot 90
            result.push(TileVersion { id, data: mirror.clone().slice_move(s![..;-1,..;-1]) });// rot 180
            result.push(TileVersion { id, data: mirror.clone().t().to_owned().slice_move(s![..;-1,..]) });// rot 270
            chunk_count += 1;
        }
        (chunk_count, result)
    }

    pub fn top(&self) -> ArrayView<char, Ix1> {
        self.data.slice(s![0,..])
    }

    pub fn bottom(&self) -> ArrayView<char, Ix1> {
        self.data.slice(s![-1,..])
    }

    pub fn left(&self) -> ArrayView<char, Ix1> {
        self.data.slice(s![..,0])
    }

    pub fn right(&self) -> ArrayView<char, Ix1> {
        self.data.slice(s![..,-1])
    }
}

impl Grid {
    fn arrange_tiles(&mut self, all_tiles: &[TileVersion], put_index: usize) -> bool {
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

    fn fits_at(&self, all_tiles: &[TileVersion], tile_index: usize, put_index: usize) -> bool {
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

    fn get_tile_id(&self, all_tiles: &[TileVersion], tile_index: usize) -> u64 {
        all_tiles[self.tiles[tile_index].unwrap()].id as u64
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let (size, tiles) = TileVersion::parse("./data/dec_20.txt");
    let grid_size = size.sqrt();
    let mut grid = Grid {
        size: grid_size,
        tiles: vec![None; grid_size * grid_size],
    };
    grid.arrange_tiles(&tiles, 0);
    println!("grid: {:?}", grid);

    let corner1 = grid.get_tile_id(&tiles, 0);
    let corner2 = grid.get_tile_id(&tiles, grid_size - 1);
    let corner3 = grid.get_tile_id(&tiles, grid_size * grid_size - 1);
    let corner4 = grid.get_tile_id(&tiles, grid_size * grid_size - grid_size);
    println!("corners: {:?} {:?} {:?} {:?}", corner1, corner2, corner3, corner4);

    let result = corner1 * corner2 * corner3 * corner4;
    println!("Result: {:?}", result);
}

pub fn part_two() {
    // return;
    // println!("--- Part Two ---");
    //
    // let tiles = Tile::parse("./data/dec_20.txt");
    // let mut grid = Grid {
    //     tiles: vec![None; GRID_SIZE * GRID_SIZE],
    // };
    // run(&tiles, &mut grid, 0);
    // println!("grid: {:?}", grid);
    //
    // let result = 0;
    // println!("Result: {:?}", result);
}


