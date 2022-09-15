use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
    purchase_price: u32,
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

    fn can_buy(&self, year: u32, funds: u32) -> bool {
        if let Ok(value) = self.buy_value(year) {
            if funds >= value {
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
    println!("input {:?}", input_path);
    let lines = common::read_strings(input_path);
    let input = Input::parse(&lines[0], &lines[1..]);
    // println!("input: {:?}", input);

    // let lifetime_0 = input.properties[0].lifetime();
    // println!("lifetime_0: {:?}", lifetime_0);
    //
    // let lifetime_1 = input.properties[1].lifetime();
    // println!("lifetime_1: {:?}", lifetime_1);
    //
    // let lifetime_2 = input.properties[2].lifetime();
    // println!("lifetime_2: {:?}", lifetime_2);
    //
    // println!("sale: {:?}", input.properties[1].for_sale(2));
    // println!("value: {:?}", input.properties[1].value(12));

    let mut money = input.seed_money;
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

    // println!("first_year: {:?}, last_year: {:?}", first_year, last_year);

    for year in first_year..last_year {
        let mut entry = OutputEntry {
            year,
            actions: vec![],
        };

        for index in 0..input.properties.len() {
            let property = &input.properties[index];

            if owned.contains_key(&index) {
                // Sell
                let record = owned.get(&index).unwrap();
                let sell_value_this_year = property.sell_value(year);
                let sell_value_next_year = property.sell_value(year + 1);
                let profit_this_year = sell_value_this_year as i32 - record.purchase_price as i32;
                let profit_next_year = sell_value_next_year as i32 - record.purchase_price as i32;
                if profit_this_year > 0 && profit_this_year >= profit_next_year {
                    entry.actions.push(Action::Sell(index));
                    owned.remove(&index);
                    money += sell_value_this_year;
                    // println!("SELL | year: {:?}, property: {:?}, value: {:?}, profit: {:?}, money: {:?}", year, property.name, sell_value_this_year, profit_this_year, money);
                }
            }

            if let Entry::Vacant(e) = owned.entry(index) {
                // Buy
                if let Ok(price) = property.buy_value(year) {
                    let sell_value_next_year = property.sell_value(year + 1);
                    let potential_next_year = sell_value_next_year as i32 - price as i32;

                    // println!("year: {:?}, property: {:?}, potential: {:?}", year, property.name, potential);

                    if potential_next_year > 0 && property.can_buy(year, money) {
                        entry.actions.push(Action::Buy(index, 0));
                        e.insert(PurchaseRecord { year, purchase_price: price });
                        money -= price;
                        // println!("BUY | year: {:?}, property: {:?}, price: {:?}, money: {:?}", year, property.name, price, money);
                    }
                }
            }
        }
        output.entries.push(entry);
    }

    let output_lines = output.serialize(&input);

    // println!("output_lines: {:?}", output_lines);

    common::write_strings(output_path, &output_lines).unwrap();
}