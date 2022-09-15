use std::fmt::format;

use crate::common;

#[derive(Clone, Debug)]
struct Input {
    seed_money: u32,
    properties: Vec<Property>,
}

#[derive(Clone, Debug)]
struct Property {
    name: String,
    construction_year: u32,
    income_property: bool,
    values: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Output {
    entries: Vec<OutputEntry>,
}

#[derive(Clone, Debug)]
struct OutputEntry {
    year: u32,
    actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Buy(usize, u32),
    Sell(usize),
    Renovate(usize),
}

impl Input {
    fn parse(first_line: &String, rest: &[String]) -> Input {
        let seed_money = first_line.parse::<u32>().unwrap();

        let properties = rest.iter()
            .map(Property::parse)
            .collect();

        Input {
            seed_money,
            properties,
        }
    }
}

impl Property {
    fn parse(line: &String) -> Property {
        let fields: Vec<String> = line.split(';')
            .map(|b| b.to_string())
            .collect();

        let construction_year = fields[1].parse::<u32>().unwrap();
        let income_property = fields[2].parse::<u32>().unwrap() == 1;

        let values = fields[3].split(',')
            .map(|b| b.to_string())
            .map(|b| b.parse::<u32>().unwrap())
            .collect();

        Property {
            name: fields[0].clone(),
            construction_year,
            income_property,
            values,
        }
    }
}

impl Output {
    fn serialize(&self, input: &Input) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for entry in &self.entries {
            result.push(entry.year.to_string());
            for action in &entry.actions {
                match action {
                    Action::Buy(index, price) => {
                        let property = &input.properties[*index];
                        result.push(format!("KAUFE {} {}", property.name, price));
                    }
                    Action::Sell(index) => {
                        let property = &input.properties[*index];
                        result.push(format!("VERKAUFE {}", property.name));
                    }
                    Action::Renovate(index) => {
                        let property = &input.properties[*index];
                        result.push(format!("RENOVIERE {}", property.name));
                    }
                }
            }
        }

        result
    }
}

pub fn run_a() {
    println!("inputA");
    let lines = common::read_strings("./data/inputA.txt");
    let input = Input::parse(&lines[0], &lines[1..]);
    println!("input: {:?}", input);

    let test_output = Output {
        entries: vec![
            OutputEntry {
                year: 0,
                actions: vec![Action::Buy(0, 0)],
            },
            OutputEntry {
                year: 2,
                actions: vec![Action::Sell(0), Action::Sell(1)],
            }],
    };

    println!("test_output: {:?}", test_output);

    let output_lines = test_output.serialize(&input);

    println!("output_lines: {:?}", output_lines);

    common::write_strings("./outputA.txt", &output_lines).unwrap();
}