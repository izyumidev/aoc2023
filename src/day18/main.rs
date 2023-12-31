use std::io::{self, Read};

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut plans: Vec<(Direction, usize)> = Vec::new();
    for line in buffer.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let dir = match split[0] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => unreachable!("Invalid direction"),
        };
        plans.push((dir, split[1].parse::<usize>().unwrap()));
    }
    let mut map: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    let mut cur_pos = (500, 500);
    map[cur_pos.0][cur_pos.1] = true;
    for plan in plans.iter() {
        match plan.0 {
            Direction::Right => {
                for _ in 0..plan.1 {
                    cur_pos.0 += 1;
                    map[cur_pos.1][cur_pos.0] = true;
                }
            }
            Direction::Left => {
                for _ in 0..plan.1 {
                    cur_pos.0 -= 1;
                    map[cur_pos.1][cur_pos.0] = true;
                }
            }
            Direction::Down => {
                for _ in 0..plan.1 {
                    cur_pos.1 += 1;
                    map[cur_pos.1][cur_pos.0] = true;
                }
            }
            Direction::Up => {
                for _ in 0..plan.1 {
                    cur_pos.1 -= 1;
                    map[cur_pos.1][cur_pos.0] = true;
                }
            }
        }
    }
    let mut count = 0;
    for line in map.iter() {
        let mut inside = false;
        let mut prev_is_digged = false;
        let mut cur = 0;
        for b in line.iter() {
            if *b {
                count += cur + 1;
                prev_is_digged = true;
                cur = 0;
            } else {
                if prev_is_digged {
                    inside = !inside;
                    prev_is_digged = false;
                }
                if inside {
                    cur += 1;
                }
            }
        }
        if *line.iter().last().unwrap() && inside {
            count += cur;
        }
    }
    println!("{}", count);
}
