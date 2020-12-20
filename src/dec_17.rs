use std::fmt;

use crate::common;

#[derive(Clone, Debug)]
struct State {
    x_size: usize,
    y_size: usize,
    z_size: usize,
    cells: Vec<Vec<Vec<u8>>>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for z in 0..self.z_size {
            write!(f, "\nz={}\n", z);
            for y in 0..self.y_size {
                for x in 0..self.x_size {
                    if self.cells[x][y][z] > 0 {
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

impl State {
    fn new(x_size: usize, y_size: usize, z_size: usize) -> State {
        return State {
            x_size,
            y_size,
            z_size,
            cells: vec![vec![vec![0; z_size]; y_size]; x_size],
        };
    }

    fn parse(filename: &str) -> State {
        let lines = common::read_strings(filename);
        let mut state = State::new(lines[0].len(), lines.len(), 1);
        let mut y = 0;
        for line in lines {
            let mut x = 0;
            for c in line.chars() {
                state.set(x, y, 0, if c == '#' { 1 } else { 0 });
                x += 1;
            }
            y += 1;
        }
        return state;
    }

    fn set(&mut self, x: i32, y: i32, z: i32, value: u8) {
        self.cells[x as usize][y as usize][z as usize] = value;
    }

    fn get(&self, x: i32, y: i32, z: i32) -> u8 {
        if x < 0 || y < 0 || z < 0
            || x >= self.x_size as i32 || y >= self.y_size as i32 || z >= self.z_size as i32 {
            return 0;
        }
        return self.cells[x as usize][y as usize][z as usize];
    }

    fn count_all(&self) -> u32 {
        let mut sum = 0u32;
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                for z in 0..self.z_size {
                    sum += self.get(x as i32, y as i32, z as i32) as u32;
                }
            }
        }
        return sum;
    }

    fn count_neighbors(&self, x: i32, y: i32, z: i32) -> u8 {
        return 0
            + self.get(x - 1, y - 1, z - 1)
            + self.get(x + 0, y - 1, z - 1)
            + self.get(x + 1, y - 1, z - 1)
            + self.get(x - 1, y + 0, z - 1)
            + self.get(x + 0, y + 0, z - 1)
            + self.get(x + 1, y + 0, z - 1)
            + self.get(x - 1, y + 1, z - 1)
            + self.get(x + 0, y + 1, z - 1)
            + self.get(x + 1, y + 1, z - 1)

            + self.get(x - 1, y - 1, z + 0)
            + self.get(x + 0, y - 1, z + 0)
            + self.get(x + 1, y - 1, z + 0)
            + self.get(x - 1, y + 0, z + 0)
            // + self.get(x + 0, y + 0, z + 0)
            + self.get(x + 1, y + 0, z + 0)
            + self.get(x - 1, y + 1, z + 0)
            + self.get(x + 0, y + 1, z + 0)
            + self.get(x + 1, y + 1, z + 0)

            + self.get(x - 1, y - 1, z + 1)
            + self.get(x + 0, y - 1, z + 1)
            + self.get(x + 1, y - 1, z + 1)
            + self.get(x - 1, y + 0, z + 1)
            + self.get(x + 0, y + 0, z + 1)
            + self.get(x + 1, y + 0, z + 1)
            + self.get(x - 1, y + 1, z + 1)
            + self.get(x + 0, y + 1, z + 1)
            + self.get(x + 1, y + 1, z + 1);
    }

    fn next(&self) -> State {
        let mut new_state = State::new(self.x_size + 2, self.y_size + 2, self.z_size + 2);
        for x in 0..new_state.x_size {
            for y in 0..new_state.y_size {
                for z in 0..new_state.z_size {
                    let old_value = self.get(x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    let count = self.count_neighbors(x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    let mut value = 0;
                    if old_value> 0 {
                        if count == 2 || count == 3 {
                            value = 1;
                        }
                    }else{
                        if count == 3 {
                            value = 1;
                        }
                    }
                    new_state.set(x as i32, y as i32, z as i32, value);
                }
            }
        }
        return new_state;
    }
}

pub fn part_one() {
    println!("--- Part One ---");
    let mut state = State::parse("./data/dec_17.txt");

    for _ in 0..6 {
        state = state.next();
    }

    println!("Result: {}", state.count_all());
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result = 0;
    println!("Result: {:?}", result);
}
