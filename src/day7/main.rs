use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, Read},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut hands: Vec<(&str, i32)> = Vec::new();
    for line in buffer.lines() {
        let a = line.split_whitespace().collect::<Vec<&str>>();
        hands.push((a[0], a[1].parse().unwrap()));
    }
    let mut played_hands: Vec<(&str, i32, HandType)> = Vec::new();
    for hand in hands.iter() {
        let mut hand_map = HashMap::new();
        let mut joker_count = 0;
        for card in hand.0.chars() {
            if card == 'J' {
                joker_count += 1;
                continue;
            }
            *hand_map.entry(card).or_insert(0) += 1;
        }
        let mut hand_type = HandType::HighCard;
        let mut hand_vec = hand_map.values().map(|&x| x).collect::<Vec<i32>>();
        hand_vec.sort();
        hand_vec.reverse();
        if hand_vec.len() == 0 {
            hand_vec.push(0)
        }
        let strongest_card = hand_vec.get_mut(0).unwrap();
        *strongest_card += joker_count;
        for v in hand_vec.iter() {
            match v {
                &2 => match hand_type {
                    HandType::OnePair => hand_type = HandType::TwoPairs,
                    HandType::ThreeOfAKind => hand_type = HandType::FullHouse,
                    _ => hand_type = HandType::OnePair,
                },
                &3 => match hand_type {
                    HandType::OnePair => hand_type = HandType::FullHouse,
                    _ => hand_type = HandType::ThreeOfAKind,
                },
                &4 => hand_type = HandType::FourOfAKind,
                &5 => hand_type = HandType::FiveOfAKind,
                _ => {}
            }
        }
        played_hands.push((hand.0, hand.1, hand_type));
    }
    played_hands.sort_by(|a, b| hand_cmp(&a.2, &b.2).then(poker_cmp(a.0, b.0)));
    let mut score = 0;
    for (i, hand) in played_hands.iter().enumerate() {
        score += (i + 1) as i32 * hand.1;
    }
    println!("{score}")
}

fn poker_cmp(a: &str, b: &str) -> Ordering {
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    let poker_alphabet = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    for i in 0..a.len() {
        if a[i] != b[i] {
            let pos_a = poker_alphabet.iter().position(|&r| r == a[i]).unwrap();
            let pos_b = poker_alphabet.iter().position(|&r| r == b[i]).unwrap();
            return pos_a.cmp(&pos_b);
        }
    }
    unreachable!()
}

fn hand_cmp(a: &HandType, b: &HandType) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let order = vec![
        HandType::HighCard,
        HandType::OnePair,
        HandType::TwoPairs,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind,
    ];
    let pos_a = order.iter().position(|r| r == a).unwrap();
    let pos_b = order.iter().position(|r| r == b).unwrap();
    pos_a.cmp(&pos_b)
}
