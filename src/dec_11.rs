use crate::common;

#[derive(Clone, Debug, PartialEq)]
enum CellState {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Clone, Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<CellState>,
}

impl Grid {
    fn parse(lines: Vec<String>) -> Grid {
        return Grid {
            height: lines.len(),
            width: lines[0].len(),
            cells: lines.iter()
                .flat_map(|line| line.chars().into_iter())
                .map(|c| match c {
                    'L' => CellState::EmptySeat,
                    '#' => CellState::OccupiedSeat,
                    _ => CellState::Floor
                })
                .collect(),
        };
    }

    pub fn get_neighbors(&self, index: usize) -> Vec<CellState> {
        let mut neighbors: Vec<CellState> = vec![];
        let x = index % self.width;
        let y = index / self.width;

        if x > 0 && y > 0 {
            neighbors.push(self.cells[(y - 1) * self.width + (x - 1)].clone());
        }
        if y > 0 {
            neighbors.push(self.cells[(y - 1) * self.width + x].clone());
        }
        if x < self.width - 1 && y > 0 {
            neighbors.push(self.cells[(y - 1) * self.width + (x + 1)].clone());
        }

        if x > 0 {
            neighbors.push(self.cells[y * self.width + (x - 1)].clone());
        }
        if x < self.width - 1 {
            neighbors.push(self.cells[y * self.width + (x + 1)].clone());
        }

        if x > 0 && y < self.height - 1 {
            neighbors.push(self.cells[(y + 1) * self.width + (x - 1)].clone());
        }
        if y < self.height - 1 {
            neighbors.push(self.cells[(y + 1) * self.width + x].clone());
        }
        if x < self.width - 1 && y < self.height - 1 {
            neighbors.push(self.cells[(y + 1) * self.width + (x + 1)].clone());
        }

        return neighbors;
    }

    pub fn next(&self) -> (Grid, bool) {
        let mut new_cells: Vec<CellState> = Vec::new();
        let mut changed = false;
        for (i, cell) in self.cells.iter().enumerate() {
            let occupied_neighbors = Grid::count_occupied(&self.get_neighbors(i));
            let new_cell_state = match cell {
                CellState::Floor => CellState::Floor,
                CellState::EmptySeat => if occupied_neighbors == 0 { CellState::OccupiedSeat } else { CellState::EmptySeat },
                CellState::OccupiedSeat => if occupied_neighbors >= 4 { CellState::EmptySeat } else { CellState::OccupiedSeat },
            };
            if new_cell_state != *cell {
                changed = true;
            }
            new_cells.push(new_cell_state);
        }
        return (Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }, changed);
    }

    pub fn count_occupied(cells: &Vec<CellState>) -> usize {
        return cells.into_iter()
            .filter(|n| **n == CellState::OccupiedSeat)
            .count();
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let mut grid = Grid::parse(common::read_strings("./data/dec_11.txt"));

    loop {
        let (new_grid, changed) = grid.next();
        if !changed {
            break;
        }
        grid = new_grid;
    }

    println!("Result: {:?}", Grid::count_occupied(&grid.cells));
}

pub fn part_two() {
    println!("--- Part One ---");


    println!("Result: {:?}", 1);
}
