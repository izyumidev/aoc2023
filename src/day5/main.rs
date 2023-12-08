use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    let mut seeds = Vec::new();
    let mut section = 0;
    let mut map: Vec<Vec<i32>> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        if line.starts_with("seeds: ") {
            line.split(' ').skip(1).for_each(|seed| {
                if seed.is_empty() {
                    return;
                }
                seeds.push(seed.parse::<i32>().unwrap());
            });
            continue;
        }
        if line.contains(':') {
            section += 1;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let mut nums = line.split(' ').map(|num| num.parse::<i32>().unwrap());
        // make vector with ith number in ith position
        if map.len() <= section {
            let mut row = Vec::new();
            for i in 0..100 {
                row.push(i)
            }
            map.push(row);
        } else {
            let mut row = map[section].clone();
            for j in 0..nums.nth(2).unwrap() {
                row[j as usize + nums.nth(1).unwrap() as usize] = nums.next().unwrap() + 1;
            }
            map[section] = row;
        }
        println!("{:?}", map)
    }
    print!("{sum}")
}

#[allow(dead_code)]
fn part_one() {}
