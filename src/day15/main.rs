use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.lines().for_each(|line| {
        // label, (box number, value)
        let mut boxes_map: HashMap<String, (usize, usize)> = HashMap::new();
        line.split(',').for_each(|s| {
            let s = s.trim();
            match s.contains('=') {
                true => {
                    let mut iter = s.split('=');
                    let label = iter.next().unwrap().trim().to_string();
                    let value = iter.next().unwrap().trim().parse::<usize>().unwrap();
                    boxes_map.entry(label).or_insert((0, 0)).1 = value;
                }
                false => {
                    let mut iter = s.split('-');
                    let label = iter.next().unwrap().trim().to_string();
                    boxes_map.remove_entry(&label);
                }
            }
            println!("{:?}", boxes_map);
        });
    });
}

#[allow(dead_code)]
fn part_one() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.lines().for_each(|line| {
        println!("{}", line.split(',').fold(0, |acc, s| acc + hash(s.trim())));
    });
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}
