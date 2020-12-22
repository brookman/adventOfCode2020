use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

#[derive(Debug, Clone)]
struct Data {
    rules: Vec<Node>,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
struct Node {
    character: Option<char>,
    alternative_1: Vec<usize>,
    alternative_2: Vec<usize>,
}

impl Data {
    pub fn parse(filename: &str) -> Data {
        let lines = common::read_strings(filename);
        let mut line_iter = lines.iter();
        let mut rules = vec![Node::new(); 1000];
        let mut messages = vec![];
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                break;
            }
            let (key, node) = Node::parse(&line);
            rules[key] = node;
        }
        while let Some(line) = line_iter.next() {
            messages.push(line.clone());
        }

        return Data {
            rules,
            messages,
        };
    }
}

impl Node {
    pub fn new() -> Node {
        return Node {
            character: None,
            alternative_1: vec![],
            alternative_2: vec![],
        };
    }
    pub fn parse(line: &String) -> (usize, Node) {
        lazy_static! {
            static ref RULE_MAIN: Regex = Regex::new(r"^(\d+)?: (.+)$").unwrap();
        }
        if RULE_MAIN.is_match(&line) {
            let caps = RULE_MAIN.captures(&line).unwrap();
            let id = caps[1].parse::<usize>().unwrap();
            let tokens = caps[2].split(" ");

            let mut node = Node::new();
            let mut list = &mut node.alternative_1;

            for token in tokens {
                if token.starts_with("\"") {
                    node.character = Some(token.chars().nth(1).unwrap());
                } else if token.starts_with("|") {
                    list = &mut node.alternative_2;
                } else {
                    list.push(token.parse::<usize>().unwrap());
                }
            }
            return (id, node);
        } else {
            panic!("PANIK");
        }
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let data = Data::parse("./data/dec_19.txt");

    let mut str = "".to_string();
    str.push('^');
    expand_to_string(&data.rules, 0, &mut str);
    str.push('$');

    println!("rule_regex: {:?}", str);
    let rule_regex = Regex::new(&str).unwrap();

    let result = data.messages.into_iter()
        .filter(|m| rule_regex.is_match(&m))
        .count();

    println!("Result: {:?}", result);
    // println!("messages: {:?}", data.messages);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result = 0;
    println!("Result: {:?}", result);
}

fn expand_to_string(nodes: &Vec<Node>, key: usize, string: &mut String) {
    let node = &nodes[key];

    if node.character.is_some() {
        string.push(node.character.unwrap());
    } else {
        string.push('(');
        for alt in &node.alternative_1 {
            expand_to_string(nodes, *alt, string);
        }
        if node.alternative_2.len() > 0 {
            string.push('|');
        }
        for alt in &node.alternative_2 {
            expand_to_string(nodes, *alt, string);
        }
        string.push(')');
    }
}
