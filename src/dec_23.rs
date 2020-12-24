#[derive(Debug, Clone)]
struct Arr {
    arr: Vec<i32>,
    present: Vec<bool>,
    min_value: i32,
    max_value: i32,
}

impl Arr {
    pub fn new(arr: &[i32]) -> Arr {
        let vec = arr.to_vec();
        let min = *vec.iter().min().unwrap();
        let max = *vec.iter().max().unwrap();
        Arr {
            arr: vec,
            present: vec![true; arr.len()],
            min_value: min,
            max_value: max,
        }
    }

    pub fn get(&self, i: i32) -> i32 {
        self.arr[Arr::wrap(i, self.arr.len())]
    }

    pub fn index_of_or_subract(&self, value_in: i32) -> Option<usize> {
        let mut value = value_in;
        if value < self.min_value {
            value = self.max_value;
        }
        for (i, v) in self.arr.iter().enumerate() {
            if *v == value {
                return if self.is_present(i as i32) {
                    Some(i)
                } else {
                    self.index_of_or_subract(value - 1)
                };
            }
        }
        None
    }

    pub fn set(&mut self, i: i32, value: i32) {
        let index = Arr::wrap(i, self.arr.len());
        self.arr[index] = value;
    }

    pub fn is_present(&self, i: i32) -> bool {
        self.present[Arr::wrap(i, self.arr.len())]
    }

    pub fn set_present(&mut self, i: i32, present: bool) {
        self.present[Arr::wrap(i, self.arr.len())] = present;
    }

    pub fn remove(&mut self, i: i32) -> i32 {
        self.present[Arr::wrap(i, self.arr.len())] = false;
        self.get(i)
    }

    pub fn insert_after(&mut self, target_i: i32, new_values: &[i32]) {
        let start_index = target_i as usize + 1;
        for (i, v) in new_values.iter().enumerate() {
            let index = start_index + i;
            self.arr.insert(index, *v);
            self.present.insert(index, true);
        }
    }

    pub fn compact(&mut self) {
        self.arr = self.arr.iter().enumerate().filter(|(i, _)| self.present[*i]).map(|(_, v)| *v).collect();
        self.present = vec![true; self.arr.len()];
    }

    fn wrap(i: i32, max_u: usize) -> usize {
        let max = max_u as i32;
        (((i % max) + max) % max) as usize
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let mut cups = Arr::new(&[1, 5, 6, 7, 9, 4, 8, 2, 3]);
    // let mut cups = Arr::new(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);

    let mut cur_i = 0;

    for _ in 0..100 {
        println!("\ncups: {:?}", &cups);
        let cur_v = cups.get(cur_i);
        let c1 = cups.remove(cur_i + 1);
        let c2 = cups.remove(cur_i + 2);
        let c3 = cups.remove(cur_i + 3);
        println!("pick up: {} {} {}", c1, c2, c3);
        let dest_i = cups.index_of_or_subract(cups.get(cur_i) - 1).unwrap();
        println!("destination: [{}] = {}", &dest_i, cups.get(dest_i as i32));
        cups.insert_after(dest_i as i32, &[c1, c2, c3]);
        println!("after insert: {:?}", &cups);
        cups.compact();
        println!("after compact: {:?}", &cups);
        cur_i = cups.index_of_or_subract(cur_v).unwrap() as i32 + 1;
    }
    let index1 = cups.index_of_or_subract(1).unwrap() + 1;
    let result = (index1..(index1 + cups.arr.len()-1)).map(|i| cups.get(i as i32).to_string()).collect::<Vec<String>>().join("");
    println!("Result: {}", result);
}

pub fn part_two() {
    println!("--- Part One ---");

    println!("Result: {:?}", 0);
}
