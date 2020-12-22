use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

struct RawData {
    rules: Vec<String>,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    Basic(u32),
    Double(u32, u32),
    Or2(u32, u32),
    Or4(u32, u32, u32, u32),
    Letter(char),
}

impl RawData {
    pub fn parse(filename: &str) -> RawData {
        let lines = common::read_strings(filename);
        let mut line_iter = lines.iter();
        let mut rules = vec![];
        let mut messages = vec![];
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                break;
            }
            rules.push(line.clone());
        }
        while let Some(line) = line_iter.next() {
            messages.push(line.clone());
        }
        return RawData {
            rules,
            messages,
        };
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let data = RawData::parse("./data/dec_19.txt");
    let mut rules: HashMap<u32, Rule> = HashMap::new();

    for rule_str in data.rules {
        lazy_static! {
            static ref RULE_MAIN: Regex = Regex::new(r"^(\d+)?: (.+)$").unwrap();
            static ref RULE_BASIC: Regex = Regex::new(r"^(\d+)?$").unwrap();
            static ref RULE_DOUBLE: Regex = Regex::new(r"^(\d+)? (\d+)?$").unwrap();
            static ref RULE_OR_2: Regex = Regex::new(r"^(\d+)? \| (\d+)?$").unwrap();
            static ref RULE_OR_4: Regex = Regex::new(r"^(\d+)? (\d+)? \| (\d+)? (\d+)?$").unwrap();
            static ref RULE_LETTER: Regex = Regex::new("^\"(.)\"$").unwrap();
        }
        if RULE_MAIN.is_match(&rule_str) {
            let caps = RULE_MAIN.captures(&rule_str).unwrap();
            let key = caps[1].parse::<u32>().unwrap();
            let content = &caps[2];
            if RULE_BASIC.is_match(&content) {
                let caps = RULE_BASIC.captures(&content).unwrap();
                rules.insert(key, Rule::Basic(
                    caps[1].parse::<u32>().unwrap()));
            } else if RULE_DOUBLE.is_match(&content) {
                let caps = RULE_DOUBLE.captures(&content).unwrap();
                rules.insert(key, Rule::Double(
                    caps[1].parse::<u32>().unwrap(),
                    caps[2].parse::<u32>().unwrap()));
            } else if RULE_OR_2.is_match(&content) {
                let caps = RULE_OR_2.captures(&content).unwrap();
                rules.insert(key, Rule::Or2(
                    caps[1].parse::<u32>().unwrap(),
                    caps[2].parse::<u32>().unwrap()));
            } else if RULE_OR_4.is_match(&content) {
                let caps = RULE_OR_4.captures(&content).unwrap();
                rules.insert(key, Rule::Or4(
                    caps[1].parse::<u32>().unwrap(),
                    caps[2].parse::<u32>().unwrap(),
                    caps[3].parse::<u32>().unwrap(),
                    caps[4].parse::<u32>().unwrap()));
            } else if RULE_LETTER.is_match(&content) {
                let caps = RULE_LETTER.captures(&content).unwrap();
                rules.insert(key, Rule::Letter(caps[1].chars().nth(0).unwrap()));
            }
        }
    }

    let mut str = "".to_string();
    str.push('^');
    expand_to_string(&rules, 0, &mut str);
    str.push('$');

    println!("rule_regex: {:?}", str);
    let rule_regex = Regex::new(&str).unwrap();

    let result = data.messages.into_iter()
        .filter(|m|rule_regex.is_match(&m))
        .count();

    println!("Result: {:?}", result);
    // println!("messages: {:?}", data.messages);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result = 0;
    println!("Result: {:?}", result);
}

fn expand_to_string(rules: &HashMap<u32, Rule>, rule_key: u32, string: &mut String) {
    let rule = rules.get(&rule_key).unwrap();
    match rule {
        Rule::Basic(v1) => {
            string.push('(');
            expand_to_string(rules, *v1, string);
            string.push(')');
        }
        Rule::Double(v1, v2) => {
            string.push('(');
            expand_to_string(rules, *v1, string);
            expand_to_string(rules, *v2, string);
            string.push(')');
        }
        Rule::Or2(v1, v2) => {
            string.push('(');
            expand_to_string(rules, *v1, string);
            string.push('|');
            expand_to_string(rules, *v2, string);
            string.push(')');
        }
        Rule::Or4(v1, v2, v3, v4) => {
            string.push('(');
            expand_to_string(rules, *v1, string);
            expand_to_string(rules, *v2, string);
            string.push('|');
            expand_to_string(rules, *v3, string);
            expand_to_string(rules, *v4, string);
            string.push(')');
        }
        Rule::Letter(c) => {
            string.push(*c);
        }
    }
}
