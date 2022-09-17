use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::common::SimpleTimer;

use crate::domain::{Action, LoanPercentage, Output, PotentialPurchase, PurchaseRecord, SimulationData};

pub fn run(input_path: &str, output_path: &str) {
    println!("input_path {:?}", input_path);

    let mut timer = SimpleTimer::start();

    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let input = SimulationData::parse(reader);

    timer.split("parsing took");

    let first_year = input.properties.iter()
        .map(|p| p.construction_year)
        .min()
        .unwrap_or(0);
    let number_of_years = input.properties.iter()
        .map(|p| p.lifetime)
        .max()
        .unwrap_or(0);
    let last_year = number_of_years - 1;

    // let interest_rates: Vec<f64> = (first_year..last_year)
    //     .map(|year| (0.2 * year as f64).sin() + (0.6 * year as f64).sin() + (0.01 * year as f64).sin() + 3.0)
    //     .collect();

    println!("properties: {:?}", input.properties.len());
    println!("first_year: {:?}, last_year: {:?}", first_year, last_year);

    let mut money = input.seed_money as f64;
    let mut owned: HashMap<usize, PurchaseRecord> = HashMap::new();

    let mut output = Output {
        actions: vec![vec![]; number_of_years as usize]
    };

    let loan = LoanPercentage::new(0).unwrap();

    for year in first_year..number_of_years {
        let year_index = year as usize;

        // Sell all:
        for index in owned.keys() {
            let property = &input.properties[*index];
            output.actions[year_index].push(Action::Sell(*index));
            money += property.sell_value(year) as f64;
        }
        owned.clear();

        let mut potential_purchases: Vec<PotentialPurchase> = vec![];

        for property_index in 0..input.properties.len() {
            let property = &input.properties[property_index];

            // Buy
            if let Ok(price) = property.buy_value(year) {
                let profit = property.sell_value(year + 1) as i32 - price as i32;

                if profit > 0 && property.can_buy(year, money, &loan) {
                    let roi = profit as f64 / price as f64;
                    potential_purchases.push(PotentialPurchase {
                        property_index,
                        price,
                        profit: profit as u32,
                        roi,
                    });
                }
            }
        }

        potential_purchases.sort_by(|a, b| b.roi.partial_cmp(&a.roi).unwrap());

        for p in potential_purchases {
            if input.properties[p.property_index].can_buy(year, money, &loan) {
                output.actions[year_index].push(Action::Buy(p.property_index, (&loan).percentage));
                owned.insert(p.property_index, PurchaseRecord { year, purchase_price: p.price as f64, loan_taken: 0.0 });
                money -= p.price as f64;
            }
        }

        // println!("year: {:?}, money: {:?}",year, money);
    }

    timer.split("calculations took");

    let output_file = File::create(output_path).unwrap();
    let mut writer = BufWriter::new(output_file);
    output.write(&input, &mut writer);

    timer.split("writing took");

    println!("money at the end: {:?}\n", money);
}