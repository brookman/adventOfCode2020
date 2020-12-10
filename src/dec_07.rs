use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

#[derive(Clone, Debug)]
struct Bags {
    map: HashMap<String, usize>,
    list: Vec<Bag>,
}

#[derive(Clone, Debug)]
struct Bag {
    index: usize,
    color: String,
    parents: Vec<BagWithQuantity>,
    children: Vec<BagWithQuantity>,
}

#[derive(Clone, Debug)]
struct BagWithQuantity {
    bag_index: usize,
    quantity: i32,
}

impl Bags {
    fn parse(strings: Vec<String>) -> Bags {
        lazy_static! {
            static ref PRIMARY: Regex = Regex::new(r"^(.+)? bags contain (.+)s?\.$").unwrap();
            static ref CHILD: Regex = Regex::new(r"^(\d) (.+)? bags?$").unwrap();
            static ref CHILD_ZERO: Regex = Regex::new(r"^no other bags$").unwrap();
        }

        let mut map: HashMap<String, usize> = HashMap::new();
        let mut list: Vec<Bag> = Vec::new();

        for string in strings {
            if !PRIMARY.is_match(&string) {
                panic!("error: {}", string);
            }
            let caps = PRIMARY.captures(&string).unwrap();
            let color = caps[1].to_string();
            let rest = caps[2].to_string();
            let child_strings = rest.split(",").map(|c| c.trim()).collect::<Vec<&str>>();

            let result = Bags::insert(map, list, &color);
            map = result.0;
            list = result.1;
            let parent = result.2;

            // let parent = list[parent_index].clone();
            let mut children: Vec<BagWithQuantity> = Vec::new();
            children.extend(parent.children.to_vec());

            for child_string in child_strings {
                if CHILD_ZERO.is_match(child_string) {
                    continue;
                }
                if !CHILD.is_match(child_string) {
                    panic!("error: {}", child_string);
                }

                let caps = CHILD.captures(child_string).unwrap();
                let quantity = caps[1].to_string().parse::<i32>().unwrap();
                let child_color = caps[2].to_string();

                let result = Bags::insert(map, list, &child_color);
                map = result.0;
                list = result.1;
                let child = result.2;

                children.push(BagWithQuantity {
                    quantity,
                    bag_index: child.index,
                });

                // let child = &list[child_index];
                let mut parents: Vec<BagWithQuantity> = Vec::new();
                parents.extend(child.parents.to_vec());
                parents.push(BagWithQuantity {
                    quantity,
                    bag_index: parent.index,
                });

                list[child.index] = Bag {
                    index: child.index,
                    color: child.color.clone(),
                    parents,
                    children: child.children.clone(),
                };
            }

            list[parent.index] = Bag {
                index: parent.index,
                color: parent.color.clone(),
                parents: parent.parents.clone(),
                children,
            };
        }
        return Bags {
            map,
            list,
        };
    }

    fn insert(mut map: HashMap<String, usize>, mut list: Vec<Bag>, color: &String) -> (HashMap<String, usize>, Vec<Bag>, Bag) {
        let index: usize;
        let bag: Bag;
        if map.contains_key(color) {
            index = *map.get(color).unwrap();
        } else {
            index = list.len();
            list.push(Bag {
                index,
                color: color.clone(),
                parents: vec![],
                children: vec![],
            });
            map.insert(color.clone(), index);
        }
        bag = list[index].clone();

        return (map, list, bag);
    }

    fn count_number_of_unique_parents_for_color(&self, color: &String) -> usize {
        let start_index = self.map.get(color).unwrap();
        let mut indices: HashSet<i32> = HashSet::new();
        indices = self.count_number_of_unique_parents_for_index(*start_index, indices);
        return indices.len();
    }

    fn count_number_of_unique_parents_for_index(&self, index: usize, mut indices: HashSet<i32>) -> HashSet<i32> {
        let bag: &Bag = self.list.get(index).unwrap();

        for parent in &bag.parents {
            indices.insert(parent.bag_index as i32);
            indices = self.count_number_of_unique_parents_for_index(parent.bag_index, indices);
        }

        return indices;
    }

    fn count_number_of_cumulative_children_for_color(&self, color: &String) -> i32 {
        let start_index = self.map.get(color).unwrap();
        return self.count_number_of_cumulative_children_for_index(*start_index) - 1;
    }

    fn count_number_of_cumulative_children_for_index(&self, index: usize) -> i32 {
        let bag: &Bag = self.list.get(index).unwrap();

        let mut total = 1;
        for child in &bag.children {
            total += child.quantity * self.count_number_of_cumulative_children_for_index(child.bag_index);
        }

        return total;
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let bags: Bags = Bags::parse(common::read_strings("./data/dec_07.txt"));
    let count = bags.count_number_of_unique_parents_for_color(&"shiny gold".to_string());
    println!("Result: {}", count);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let bags: Bags = Bags::parse(common::read_strings("./data/dec_07.txt"));
    let count = bags.count_number_of_cumulative_children_for_color(&"shiny gold".to_string());
    println!("Result: {}", count);
}
