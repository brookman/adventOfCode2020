use std::slice::Iter;

use crate::common;

#[derive(Debug, Clone)]
pub enum LexItem {
    Parenthesis(char),
    Operation(char),
    Digit(u64),
}

pub fn part_one() {
    println!("--- Part One ---");

    let result = common::read_strings("./data/dec_18.txt").iter()
        .map(|line| {
            let items = lex(line).unwrap();
            return evaluate(&mut items.iter());
        }).sum();

    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result = 0;
    println!("Result: {:?}", result);
}

fn evaluate(items: &mut Iter<LexItem>) -> u64 {
    let mut value = 0u64;
    let mut last_value = 0u64;
    let mut last_operation = '+';
    while let Some(item) = items.next() {
        match item {
            LexItem::Parenthesis(c) => {
                if *c == '(' {
                    last_value = evaluate(items);
                } else if *c == ')' {
                    return value;
                }
            }
            LexItem::Operation(c) => {
                last_operation = *c;
                continue;
            }
            LexItem::Digit(d) => {
                last_value = *d;
            }
        }
        match last_operation {
            '+' => value += last_value,
            '*' => value *= last_value,
            _ => {}
        }
    }
    return value;
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars();
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                let n = c.to_string().parse::<u64>().unwrap();
                result.push(LexItem::Digit(n));
            }
            '+' | '*' => {
                result.push(LexItem::Operation(c));
            }
            '(' | ')' => {
                result.push(LexItem::Parenthesis(c));
            }
            ' ' => {}
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}
