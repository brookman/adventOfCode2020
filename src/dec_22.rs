use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let data = common::read_strings("./data/dec_22.txt");
    let mut player1 = data[1..26].iter().rev().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut player2 = data[28..].iter().rev().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop().unwrap();
        let card2 = player2.pop().unwrap();

        if card1 > card2 {
            player1.insert(0, card1);
            player1.insert(0, card2);
        } else {
            player2.insert(0, card2);
            player2.insert(0, card1);
        }
    }

    if player1.is_empty() {
        print_result(&player2);
    } else if player2.is_empty() {
        print_result(&player1);
    }
}

pub fn part_two() {
    println!("--- Part One ---");

    println!("Result: {:?}", 0);
}

fn print_result(hand: &Vec<i32>) {
    let result: i32 = hand.iter().enumerate().map(|(i, h)| (i as i32 + 1) * h).sum();
    println!("Result: {:?}", result);
}
