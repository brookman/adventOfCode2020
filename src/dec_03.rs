use crate::common;

struct Landscape {
    grid: Vec<Vec<i32>>
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
        let mut grid: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            let line_as_numbers: Vec<i32> = line.chars().into_iter()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0
                })
                .collect();
            grid.push(line_as_numbers);
        }
        return Landscape {
            grid,
        };
    }

    fn evaluate(&self, slope: &Slope) -> i32 {
        let height = self.grid.len();
        let width = self.grid[0].len();

        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        while y < height {
            count += self.grid[y][x];
            x = (x + slope.x) % width;
            y = y + slope.y;
        }
        return count;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let landscape = Landscape::from_file("./data/dec_03.txt");
    let result = landscape.evaluate(&Slope { x: 3, y: 1 });
    println!("Result: {}", result)
}

pub fn part_two() {
    println!("--- Part Two ---");

    let landscape = Landscape::from_file("./data/dec_03.txt");
    let results = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 }].iter()
        .map(|s| landscape.evaluate(s))
        .collect::<Vec<i32>>();
    println!("Result: {}", common::format_to_product(&results));
}
