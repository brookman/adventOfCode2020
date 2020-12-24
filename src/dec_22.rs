use std::collections::HashSet;

use crate::common;
use crate::dec_22::Player::Player1;
use crate::dec_22::Player::Player2;

enum Player {
    Player1,
    Player2,
}

pub fn part_one() {
    println!("--- Part One ---");

    let (mut player1, mut player2) = get_hands("./data/dec_22.txt");

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

    let winner_cards = if player1.is_empty() { player2 } else { player1 };
    println!("Result: {:?}", sum(&winner_cards));
}

pub fn part_two() {
    println!("--- Part Two ---");

    let (player1, player2) = get_hands("./data/dec_22.txt");
    let (_, winner_cards) = recurse(&player1, &player2);
    println!("Result: {:?}", sum(&winner_cards));
}

fn recurse(player1_in: &[usize], player2_in: &[usize]) -> (Player, Vec<usize>) {
    let mut player1 = player1_in.to_vec();
    let mut player2 = player2_in.to_vec();
    let mut old_hands: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        let hand = (player1.clone(), player2.clone());
        if old_hands.contains(&hand) {
            return (Player1, player1);
        } else {
            old_hands.insert(hand);
        }

        let card1 = player1.pop().unwrap();
        let card2 = player2.pop().unwrap();

        let winner =
            if player1.len() >= card1 && player2.len() >= card2 {
                recurse(&player1[player1.len() - card1..], &player2[player2.len() - card2..]).0
            } else if card1 > card2 { Player1 } else { Player2 };

        match winner {
            Player1 => {
                player1.insert(0, card1);
                player1.insert(0, card2);
            }
            Player2 => {
                player2.insert(0, card2);
                player2.insert(0, card1);
            }
        }
    }
    if player1.is_empty() { (Player2, player2) } else { (Player1, player1) }
}

fn get_hands(filename: &str) -> (Vec<usize>, Vec<usize>) {
    let data = common::read_strings(filename);
    let player1 = data[1..26].iter().rev().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let player2 = data[28..].iter().rev().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    (player1, player2)
}

fn sum(hand: &[usize]) -> usize {
    return hand.iter().enumerate().map(|(i, h)| (i as usize + 1) * h).sum();
}
