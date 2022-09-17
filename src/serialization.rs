use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::domain::{Action, Output, Property, SimulationData};

impl SimulationData {
    pub fn parse(reader: BufReader<File>) -> SimulationData {
        let mut lines = reader.lines().flatten();

        let seed_money = lines.next().unwrap().parse::<u32>().unwrap();
        let properties = lines
            .map(Property::parse)
            .collect();

        SimulationData {
            seed_money,
            properties,
        }
    }
}

impl Property {
    pub fn parse(line: String) -> Property {
        let fields: Vec<String> = line.split(';')
            .map(|b| b.to_string())
            .collect();

        let name = fields[0].clone();
        let construction_year = fields[1].parse::<u32>().unwrap();
        let income_property = fields[2].parse::<u32>().unwrap() == 1;

        let values: Vec<u32> = fields[3].split(',')
            .map(|b| b.to_string())
            .map(|b| b.parse::<u32>().unwrap())
            .collect();
        let len = values.len() as u32;

        Property {
            name,
            construction_year,
            income_property,
            values,
            lifetime: construction_year + len,
        }
    }
}

impl Output {
    pub fn write(&self, input: &SimulationData, writer: &mut BufWriter<File>) {
        for year_index in 0usize..self.actions.len() {
            let actions = &self.actions[year_index];
            if actions.is_empty() {
                continue;
            }

            writer.write_all(format!("{}\n", year_index).as_bytes()).unwrap();
            for action in actions {
                match action {
                    Action::Buy(index, finance_percentage) => {
                        let property = &input.properties[*index];
                        writer.write_all(format!("KAUFE {} {}\n", property.name, finance_percentage).as_bytes()).unwrap();
                    }
                    Action::Sell(index) => {
                        let property = &input.properties[*index];
                        writer.write_all(format!("VERKAUFE {}\n", property.name).as_bytes()).unwrap();
                    }
                    // Action::Renovate(index) => {
                    //     let property = &input.properties[*index];
                    //     writer.write_all(format!("RENOVIERE {}\n", property.name).as_bytes()).unwrap();
                    // }
                }
            }
        }
    }
}