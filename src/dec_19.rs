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
    Alternative(Vec<Rule>),
    Sequence(Vec<Rule>),
    Character(char),
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
        for line in line_iter {
            messages.push(line.clone());
        }
        RawData {
            rules,
            messages,
        }
    }
}

impl Rule {
    pub fn parse(rules: &HashMap<usize, String>, index: usize) -> Rule {
        let rule_str = rules.get(&index).unwrap();
        Rule::parse_str(rules, rule_str)
    }

    fn parse_str(rules: &HashMap<usize, String>, string: &String) -> Rule {
        if string.contains('|') {
            let mut alternatives = vec![];
            for alternative_str in string.split('|') {
                alternatives.push(Rule::parse_str(rules, &alternative_str.trim().to_string()));
            }
            Rule::Alternative(alternatives)
        } else if string.contains(' ') {
            let mut sequences = vec![];
            for sequence_str in string.split(' ') {
                sequences.push(Rule::parse_str(rules, &sequence_str.trim().to_string()));
            }
            Rule::Sequence(sequences)
        } else if string.contains('"') {
            Rule::Character(string.chars().nth(1).unwrap())
        } else {
            Rule::parse(rules, string.parse::<usize>().unwrap())
        }
    }
}

pub fn part_one() {
    lazy_static! {
        static ref RULE_MAIN: Regex = Regex::new(r"^(\d+)?: (.+)$").unwrap();
    }

    println!("--- Part One ---");

    let data = RawData::parse("./data/dec_19.txt");
    let mut raw_rules: HashMap<usize, String> = HashMap::new();

    for rule_str in &data.rules {
        if RULE_MAIN.is_match(rule_str) {
            let caps = RULE_MAIN.captures(rule_str).unwrap();
            let key = caps[1].parse::<usize>().unwrap();
            let content = caps[2].parse::<String>().unwrap();
            raw_rules.insert(key, content);
        }
    }

    let rule = Rule::parse(&raw_rules, 0);


    let mut str = "".to_string();
    str.push('^');
    expand_to_string(&rule,  &mut str);
    str.push('$');

    let rule_regex = Regex::new(&str).unwrap();

    let result = data.messages.into_iter()
        .filter(|m| rule_regex.is_match(&m))
        .count();
    println!("Result: {:?}", result);
}

pub fn part_two() {
    lazy_static! {
        static ref RULE_MAIN: Regex = Regex::new(r"^(\d+)?: (.+)$").unwrap();
    }

    println!("--- Part One ---");

    let data = RawData::parse("./data/dec_19.txt");
    let mut raw_rules: HashMap<usize, String> = HashMap::new();

    for rule_str in &data.rules {
        if RULE_MAIN.is_match(rule_str) {
            let caps = RULE_MAIN.captures(rule_str).unwrap();
            let key = caps[1].parse::<usize>().unwrap();
            let content = caps[2].parse::<String>().unwrap();
            raw_rules.insert(key, content);
        }
    }

    raw_rules.insert(8,"42 | 42 42 | 42 42 42 | 42 42 42 42 | 42 42 42 42 42 | 42 42 42 42 42 42 | 42 42 42 42 42 42 42".to_string());
    raw_rules.insert(11,"42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31".to_string());

    let rule = Rule::parse(&raw_rules, 0);

    let mut str = "".to_string();
    str.push('^');
    expand_to_string(&rule,  &mut str);
    str.push('$');

    let rule_regex = Regex::new(&str).unwrap();

    let result = data.messages.into_iter()
        .filter(|m| rule_regex.is_match(&m))
        .count();
    println!("Result: {:?}", result);
}

fn expand_to_string(rule: &Rule, string: &mut String) {
    match rule {
        Rule::Alternative(alternatives) => {
            string.push('(');
            for (i, alternative) in alternatives.iter().enumerate() {
                expand_to_string(alternative, string);
                if i < alternatives.len() - 1 {
                    string.push('|');
                }
            }
            string.push(')');
        }
        Rule::Sequence(sequences) => {
            string.push('(');
            for sequence in sequences {
                expand_to_string(sequence, string);
            }
            string.push(')');
        }
        Rule::Character(c) => {
            string.push(*c);
        }
    }
}
