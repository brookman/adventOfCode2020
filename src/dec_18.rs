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

    let result: u64 = common::read_strings("./data/dec_18.txt").iter()
        .map(|line| {
            let items = lex(line).unwrap();
            return evaluate(&mut items.iter());
        }).sum();

    println!("Result: {:?}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let result: u64 = common::read_strings("./data/dec_18.txt").iter()
        .map(|line| {
            let items = lex(line).unwrap();
            return evaluate2(&mut items.iter());
        }).sum();

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

fn evaluate2(items: &mut Iter<LexItem>) -> u64 {
    let mut value_stack: Vec<u64> = vec![];
    let mut operator_stack: Vec<char> = vec![];

    while let Some(item) = items.next() {
        match item {
            LexItem::Parenthesis('(') => {
                operator_stack.push('(');
            }
            LexItem::Parenthesis(')') => {
                while *operator_stack.last().unwrap() != '(' {
                    execute_operation(&mut value_stack, &mut operator_stack);
                }
                operator_stack.pop();
            }
            LexItem::Operation(c) => {
                let this_op = c;
                while operator_stack.len() > 0
                    && precedence(&operator_stack.last().unwrap()) >= precedence(&this_op) {
                    execute_operation(&mut value_stack, &mut operator_stack);
                }
                operator_stack.push(*this_op);
            }
            LexItem::Digit(d) => {
                value_stack.push(*d);
            }
            _ => {}
        }
    }

    while operator_stack.len() > 0 {
        execute_operation(&mut value_stack, &mut operator_stack);
    }

    return value_stack.pop().unwrap();
}

fn execute_operation(value_stack: &mut Vec<u64>, operator_stack: &mut Vec<char>) {
    let op: char = operator_stack.pop().unwrap();
    let value_1 = value_stack.pop().unwrap();
    let value_2 = value_stack.pop().unwrap();
    value_stack.push(match op {
        '+' => value_1 + value_2,
        '*' => value_1 * value_2,
        _ => 0
    });
}

fn precedence(op: &char) -> i32 {
    return match op {
        '+' => 2,
        '*' => 1,
        _ => 0,
    };
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
