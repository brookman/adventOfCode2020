use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;
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

#[derive(Debug, Clone)]
struct PurchaseRecord {
    year: u32,
    purchase_price: f64,
    loan: f64,
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

    fn lifetime(&self) -> u32 {
        self.construction_year + self.values.len() as u32
    }

    fn for_sale(&self, year: u32) -> bool {
        year >= self.construction_year && year < self.lifetime()
    }

    fn buy_value(&self, year: u32) -> Result<u32, String> {
        if !self.for_sale(year) {
            return Err(format!("Not for sale in year {}", year));
        }
        // println!("buy_value: {:?}, construction_year {:?}", year, self.construction_year);
        Ok(self.values[(year - self.construction_year) as usize])
    }

    fn sell_value(&self, year: u32) -> u32 {
        if self.for_sale(year) {
            return self.values[(year - self.construction_year) as usize];
        }
        0
    }

    fn can_buy(&self, year: u32, funds: f64, loan_percentage: u32) -> bool {
        if let Ok(value) = self.buy_value(year) {
            let price_to_pay = (100 - loan_percentage) as f64 * value as f64 / 100.0;
            if funds >= price_to_pay as f64 {
                return true;
            }
        }
        false
    }
}

impl Output {
    fn serialize(&self, input: &Input) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for entry in &self.entries {
            if entry.actions.is_empty() {
                continue;
            }
            result.push(entry.year.to_string());
            for action in &entry.actions {
                match action {
                    Action::Buy(index, finance_percentage) => {
                        let property = &input.properties[*index];
                        result.push(format!("KAUFE {} {}", property.name, finance_percentage));
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

pub fn run(input_path: &str, output_path: &str) {
    let start = Instant::now();

    println!("input_path {:?}", input_path);
    let lines = common::read_strings(input_path);
    let input = Input::parse(&lines[0], &lines[1..]);

    let mut money = input.seed_money as f64;
    let mut loans = 0f64;
    let mut owned: HashMap<usize, PurchaseRecord> = HashMap::new();
    let mut output = Output {
        entries: vec![]
    };

    let first_year = input.properties.iter()
        .map(|p| p.construction_year)
        .min()
        .unwrap_or(0);

    let last_year = input.properties.iter()
        .map(|p| p.lifetime() - 1)
        .max()
        .unwrap_or(0);

    println!("properties: {:?}", input.properties.len());
    println!("first_year: {:?}, last_year: {:?}", first_year, last_year);

    for year in first_year..last_year {
        let mut entry = OutputEntry {
            year,
            actions: vec![],
        };

        for index in 0..input.properties.len() {
            let property = &input.properties[index];

            let interest_rate = (0.2 * year as f64).sin() + (0.6 * year as f64).sin() + (0.01 * year as f64).sin() + 3.0;
            let interest = loans * interest_rate / 100.0;
            // println!("interest: {:?}", interest);
            money -= interest;
            if money < 0.0 {
                panic!("failed");
            }

            if owned.contains_key(&index) {
                // Sell
                let record = owned.get(&index).unwrap();
                let sell_value_this_year = property.sell_value(year);
                let sell_value_next_year = property.sell_value(year + 1);
                let profit_this_year = sell_value_this_year as i32 - record.purchase_price as i32;
                let profit_next_year = sell_value_next_year as i32 - record.purchase_price as i32;
                if profit_this_year > 0 && profit_this_year >= profit_next_year {
                    entry.actions.push(Action::Sell(index));
                    loans -= record.loan;
                    money += sell_value_this_year as f64;
                    owned.remove(&index);
                    // println!("SELL | year: {:?}, property: {:?}, value: {:?}, profit: {:?}, money: {:?}", year, property.name, sell_value_this_year, profit_this_year, money);
                }
            }

            if let Entry::Vacant(e) = owned.entry(index) {
                // Buy
                if let Ok(price) = property.buy_value(year) {
                    let sell_value_next_year = property.sell_value(year + 1);
                    let potential_next_year = sell_value_next_year as i32 - price as i32;

                    // println!("year: {:?}, property: {:?}, potential: {:?}", year, property.name, potential);

                    let loan_percentage = 0;
                    let out_of_pocket = (100 - loan_percentage) as f64 * price as f64 / 100.0;
                    let loan = loan_percentage as f64 * price as f64 / 100.0;

                    if potential_next_year > 0 && property.can_buy(year, money, loan_percentage) {
                        entry.actions.push(Action::Buy(index, loan_percentage));

                        e.insert(PurchaseRecord { year, purchase_price: out_of_pocket, loan });
                        money -= out_of_pocket;
                        loans += loan;
                        // println!("BUY | year: {:?}, property: {:?}, price: {:?}, money: {:?}, loans: {:?}", year, property.name, out_of_pocket, money, loans);
                    }
                }
            }
        }
        output.entries.push(entry);
    }

    let output_lines = output.serialize(&input);

    // println!("output_lines: {:?}", output_lines);

    common::write_strings(output_path, &output_lines).unwrap();

    println!("duration: {:?}", start.elapsed());
}