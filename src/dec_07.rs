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
    color: String,
    parents: Vec<ParentConnection>,
}

#[derive(Clone, Debug)]
struct ParentConnection {
    bag_index: usize,
    quantity: i32,
}

#[derive(Clone, Debug)]
struct ChildBag {
    color: String,
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
            if PRIMARY.is_match(&string) {
                let caps = PRIMARY.captures(&string).unwrap();
                let color = caps[1].to_string();
                let rest = caps[2].to_string();
                let child_strings = rest.split(",").map(|c| c.trim()).collect::<Vec<&str>>();
                let mut children: Vec<ChildBag> = Vec::new();

                for child_string in child_strings {
                    if CHILD.is_match(child_string) {
                        let caps = CHILD.captures(child_string).unwrap();
                        let quantity = caps[1].to_string().parse::<i32>().unwrap();
                        let child_color = caps[2].to_string();
                        children.push(ChildBag {
                            color: child_color,
                            quantity,
                        });
                    } else if CHILD_ZERO.is_match(child_string) {} else {
                        panic!("error: {}", child_string);
                    }
                }

                let parent_index: usize;
                if map.contains_key(&color) {
                    parent_index = *map.get(&color).unwrap();
                } else {
                    list.push(Bag {
                        color: color.clone(),
                        parents: vec![],
                    });
                    map.insert(color.clone(), list.len() - 1);
                    parent_index = list.len() - 1;
                }

                for child in children {
                    let child_index: usize;
                    if map.contains_key(&child.color) {
                        child_index = *map.get(&child.color).unwrap();
                    } else {
                        list.push(Bag {
                            color: child.color.clone(),
                            parents: vec![],
                        });
                        map.insert(child.color.clone(), list.len() - 1);
                        child_index = list.len() - 1;
                    }
                    let old_child = &list[child_index];
                    let mut parents: Vec<ParentConnection> = Vec::new();
                    parents.extend(old_child.parents.to_vec());
                    parents.push(ParentConnection {
                        quantity: child.quantity,
                        bag_index: parent_index,
                    });

                    let new_child = Bag {
                        color: old_child.color.clone(),
                        parents,
                    };
                    list[child_index] = new_child;
                }
            } else {
                panic!();
            }
        }
        return Bags {
            map,
            list,
        };
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
}

pub fn part_one() {
    println!("--- Part One ---");
    let bags: Bags = Bags::parse(common::read_strings("./data/dec_07.txt"));
    let count = bags.count_number_of_unique_parents_for_color(&"shiny gold".to_string());
    println!("Result: {}", count);
}

pub fn part_two() {
    println!("--- Part Two ---");
}
