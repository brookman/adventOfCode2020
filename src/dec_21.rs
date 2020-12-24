use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use itertools::Itertools;

use crate::common;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    pub fn parse(filename: &str) -> Vec<Food> {
        let mut result = Vec::new();
        for line in common::read_strings(filename) {
            let parts: Vec<&str> = line.split('(').collect();
            let ingredients: Vec<String> = parts[0].trim().split_whitespace()
                .map(|i| i.trim().to_string())
                .filter(|i| !i.is_empty())
                .collect();
            let allergens: Vec<String> = parts[1].replace(",", "")
                .replace(")", "")
                .replace("contains", "")
                .split_whitespace()
                .map(|i| i.trim().to_string())
                .filter(|i| !i.is_empty())
                .collect();
            result.push(Food {
                ingredients: HashSet::<String>::from_iter(ingredients.iter().cloned()),
                allergens: HashSet::<String>::from_iter(allergens.iter().cloned()),
            });
        }
        result
    }
    pub fn remove(&mut self, allergen: &String, ingredient: &String) {
        self.allergens.remove(allergen);
        self.ingredients.remove(ingredient);
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let mut foods = Food::parse("./data/dec_21.txt");
    let mut map = HashMap::<String, String>::new();

    loop {
        let it_result = do_iteration(&foods);
        if it_result.is_some() {
            let (allergen, ingredient) = it_result.unwrap();
            for food in &mut foods {
                food.remove(&allergen, &ingredient);
            }
            map.insert(allergen, ingredient);
        } else {
            break;
        }
    }

    let result: usize = foods.iter().map(|f| f.ingredients.len()).sum();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut foods = Food::parse("./data/dec_21.txt");
    let mut map = HashMap::<String, String>::new();

    loop {
        let it_result = do_iteration(&foods);
        if it_result.is_some() {
            let (allergen, ingredient) = it_result.unwrap();
            for food in &mut foods {
                food.remove(&allergen, &ingredient);
            }
            map.insert(allergen, ingredient);
        } else {
            break;
        }
    }

    let mut dangerous_ingredients: Vec<_> = map.into_iter().collect();
    dangerous_ingredients.sort_by(|x, y| x.0.cmp(&y.0));
    let result = dangerous_ingredients.iter().map(|(k, v)| v).join(",");
    println!("Result: {}", result);
}

fn do_iteration(foods: &Vec<Food>) -> Option<(String, String)> {
    let unique_allergens = HashSet::<String>::from_iter(foods.iter().flat_map(|f| f.allergens.clone().into_iter()));
    let unique_ingredients = HashSet::<String>::from_iter(foods.iter().flat_map(|f| f.ingredients.clone().into_iter()));
    for allergen in unique_allergens {
        let mut food_with_allergen: Vec<Food> = vec![];
        for food in foods {
            if food.allergens.contains(&allergen) {
                food_with_allergen.push(food.clone());
            }
        }
        let correlation = find_correlation(&food_with_allergen, &unique_ingredients);
        if correlation.is_some() {
            return Some((allergen, correlation.unwrap()));
        }
    }
    None
}

fn find_correlation(food_with_allergen: &Vec<Food>, unique_ingredients: &HashSet<String>) -> Option<String> {
    let matches: Vec<&String> = unique_ingredients.iter().filter(|ingredient| {
        food_with_allergen.iter().all(|food| food.ingredients.contains(*ingredient))
    }).collect();
    if matches.len() == 1 {
        return Some(matches[0].clone());
    }
    None
}
