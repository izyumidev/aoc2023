use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    let mut card = vec![1; buffer.lines().count() + 1];
    for (i, line) in buffer.lines().enumerate() {
        let numbers = line.split(':').last().unwrap().trim().to_string();
        let left = numbers
            .split('|')
            .nth(0)
            .unwrap()
            .trim()
            .to_string()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let right = numbers
            .split('|')
            .nth(1)
            .unwrap()
            .trim()
            .to_string()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut overlaps = 0;
        for i in 0..left.len() {
            for j in 0..right.len() {
                if left[i] == right[j] {
                    overlaps += 1;
                }
            }
        }
        for j in 0..overlaps {
            card[i + j + 1] += card[i];
        }
    }
    for i in 0..card.len() - 1 {
        sum += card[i];
    }
    println!("{}", sum);
}
