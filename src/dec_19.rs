use std::cmp;
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
        let mut rules_map = HashMap::new();
        let mut max = 0;

        let mut messages = vec![];
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                break;
            }
            let (key, node) = Node::parse(&line);
            max = cmp::max(max, key);
            rules_map.insert(key, node);
        }
        while let Some(line) = line_iter.next() {
            messages.push(line.clone());
        }

        let mut rules = vec![Node::new(); max + 1];
        for (key, value) in rules_map {
            rules[key] = value;
        }
        Data {
            rules,
            messages,
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            character: None,
            alternative_1: vec![],
            alternative_2: vec![],
        }
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

    // let data = Data::parse("./data/dec_19.txt");
    //
    // let result = data.messages.iter()
    //     .filter(|m| is_valid(&data.rules, m))
    //     .count();
    // println!("Result: {:?}", result);
}

pub fn part_two() {
    // println!("--- Part Two ---");
    //
    // let mut data = Data::parse("./data/dec_19.txt");
    //
    // data.rules[8] = Node {
    //     character: None,
    //     alternative_1: vec![42],
    //     alternative_2: vec![42, 8],
    // };
    // data.rules[11] = Node {
    //     character: None,
    //     alternative_1: vec![42, 31],
    //     alternative_2: vec![42, 11, 31],
    // };
    //
    // let result = data.messages.iter()
    //     .filter(|m| is_valid(&data.rules, m))
    //     .count();
    // println!("Result: {:?}", result);
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

fn is_valid(nodes: &Vec<Node>, string: &String) -> bool {
    let (used, matches) = check_valid(nodes, 0, string, 0);
    let valid = matches && used + 1 == string.len();
    println!("{:?} valid: {}, matches: {}, used: {}", string, valid, matches, used);
    return matches && used + 1 == string.len();
}

fn check_valid(nodes: &Vec<Node>, node_key: usize, string: &String, char_index: usize) -> (usize, bool) {
    if char_index >= string.len() {
        println!("exit overrun {:?}", (0, false));
        return (0, false);
    }

    let node = &nodes[node_key];
    let char = string.chars().nth(char_index).unwrap();
    println!("node[{}]={:?}, char[{}]={}", node_key, node, char_index, char);
    match node.character {
        Some(node_char) => {
            println!("exit match {:?}", (0, node_char == char));
            return (1, node_char == char);
        }
        None => {
            let res1 = check_sequence(nodes, &node.alternative_1, string, char_index);
            if res1.1 {
                println!("exit res1 {:?}", res1);
                return res1;
            }
            let res2 = check_sequence(nodes, &node.alternative_2, string, char_index);
            if res2.1 {
                println!("exit res2 {:?}", res1);
                return res2;
            }
        }
    }
    println!("exit end {:?}", (0, false));
    return (0, false);
}

fn check_sequence(nodes: &Vec<Node>, seq: &Vec<usize>, string: &String, char_index: usize) -> (usize, bool) {
    if seq.len() > 0 {
        let mut all_match = true;
        let mut offset = 0;
        for (i, alt) in seq.iter().enumerate() {
            let (used, matches) = check_valid(nodes, *alt, string, char_index + i + offset);
            all_match &= matches;
            if !matches {
                break;
            }
            offset += used;
        }
        if all_match {
            return (seq.len() - 1 + offset, true);
        }
    }
    return (0, false);
}
