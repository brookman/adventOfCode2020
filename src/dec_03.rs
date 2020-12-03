use crate::common;

struct Landscape {
    landscape: Vec<Vec<i32>>
}

struct Slope {
    x: usize,
    y: usize,
}

impl Landscape {
    fn from_file(filename: &str) -> Landscape {
        let lines = common::read_strings(filename);
        return Landscape::parse(&lines);
    }

    fn parse(lines: &Vec<String>) -> Landscape {
        let mut landscape: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            let line_as_numbers: Vec<i32> = line.chars().into_iter()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0
                })
                .collect();
            landscape.push(line_as_numbers);
        }
        return Landscape {
            landscape,
        };
    }

    fn evaluate(&self, slope: &Slope) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        while y < self.landscape.len() {
            count += self.landscape[y][x];
            x = (x + slope.x) % self.landscape[0].len();
            y = y + slope.y;
        }
        return count as i64;
    }

    fn evaluate_slopes_and_multiply_results(&self, slopes: &Vec<Slope>) -> i64 {
        return slopes.iter()
            .map(|s| self.landscape.evaluate(s))
            .product();
    }
}

pub fn part_one() {
    println!("--- Part One ---");
    let landscape = Landscape::from_file("./data/dec_03.txt");
    let count = landscape.evaluate(&Slope { x: 3, y: 1 });
    println!("Result: {}", count)
}


pub fn part_two() {
    println!("--- Part Two ---");
    let landscape = Landscape::from_file("./data/dec_03.txt");
    let result = landscape.evaluate_slopes_and_multiply_results([
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 }]);

    println!("Result: {}", result);
}
