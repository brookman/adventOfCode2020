use itertools::Itertools;

use crate::common;

#[derive(Clone, Debug)]
struct State {
    last: i32,
    diff_1: i32,
    diff_3: i32,
}

impl State {
    fn new() -> State {
        return State {
            last: 0,
            diff_1: 0,
            diff_3: 0,
        };
    }
    pub fn next(&self, current: i32) -> State {
        let diff = current - self.last;
        return match diff {
            1 => {
                State {
                    last: current,
                    diff_1: self.diff_1 + 1,
                    diff_3: self.diff_3,
                }
            }
            3 => {
                State {
                    last: current,
                    diff_1: self.diff_1,
                    diff_3: self.diff_3 + 1,
                }
            }
            _ => {
                State {
                    last: current,
                    diff_1: self.diff_1,
                    diff_3: self.diff_3,
                }
            }
        };
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let state: State = common::read_numbers("./data/dec_10.txt").into_iter()
        .sorted()
        .fold(State::new(), |state, current| state.next(current));
    let result = state.diff_1 * (state.diff_3 + 1);

    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let numbers: Vec<i32> = common::read_numbers("./data/dec_10.txt").into_iter()
        .sorted()
        .collect();

    let mut paths: Vec<i64> = vec![0; numbers.len()];

    for (i, number) in numbers.iter().enumerate() {
        paths[i] = get_paths(&numbers, &paths, number, i as i32 - 1) +
            get_paths(&numbers, &paths, number, i as i32 - 2) +
            get_paths(&numbers, &paths, number, i as i32 - 3);
    }

    println!("Result: {:?}", paths[paths.len() - 1]);
}

fn get_paths(numbers: &Vec<i32>, paths: &Vec<i64>, number: &i32, target: i32) -> i64 {
    let (target_number, n_paths) = match target {
        -1 => (0, 1),
        -2 => (0, 0),
        -3 => (0, 0),
        _ => (numbers[target as usize], paths[target as usize])
    };
    if number - target_number <= 3 {
        return n_paths;
    }
    return 0;
}
