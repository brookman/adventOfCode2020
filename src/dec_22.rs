use crate::common;

pub fn part_one() {
    println!("--- Part One ---");
    let data = common::read_strings("./data/dec_22.txt");
    let mut player1 = data[1..26].iter().rev().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut player2 = data[28..].iter().rev().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("player1: {:?}", player1);
    println!("player2: {:?}", player2);

    loop {
        let card1 = player1.pop();
        let card2 = player2.pop();

        if card1.unwrap() > card2.unwrap() {
            player1.insert(0, card1.unwrap());
            player1.insert(0, card2.unwrap());
        } else {
            player2.insert(0, card2.unwrap());
            player2.insert(0, card1.unwrap());
        }

        println!("player1: {:?}", player1);
        println!("player2: {:?}", player2);

        if player1.is_empty() {
            println!("player2 won");
            print_result(&player2);
            break;
        }
        if player2.is_empty() {
            println!("player1 won");
            print_result(&player1);
            break;
        }
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
