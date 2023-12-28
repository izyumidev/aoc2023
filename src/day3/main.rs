use std::io::{self, Read};

#[derive(Debug)]
struct Number {
    val: usize,
    points: Vec<(i32, i32)>,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<(i32, i32)> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        let mut cur_val = 0;
        let mut cur_points: Vec<(i32, i32)> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                cur_val = cur_val * 10 + c.to_digit(10).unwrap() as usize;
                cur_points.push((x as i32, y as i32));
                continue;
            }
            if cur_val > 0 {
                numbers.push(Number {
                    val: cur_val,
                    points: cur_points.clone(),
                });
                cur_val = 0;
                cur_points = Vec::new();
            }
            if c.eq(&'*') {
                gears.push((x as i32, y as i32));
            }
        }
        if cur_val > 0 {
            numbers.push(Number {
                val: cur_val,
                points: cur_points.clone(),
            });
        }
    }
    for gear in gears.iter() {
        let mut count = 0;
        let mut prod = 1;
        for num in numbers.iter() {
            let to_check = vec![
                (gear.0 - 1, gear.1 - 1),
                (gear.0 - 1, gear.1),
                (gear.0 - 1, gear.1 + 1),
                (gear.0, gear.1 - 1),
                (gear.0, gear.1 + 1),
                (gear.0 + 1, gear.1 - 1),
                (gear.0 + 1, gear.1),
                (gear.0 + 1, gear.1 + 1),
            ];
            'inner: for check in to_check.iter() {
                if num.points.contains(check) {
                    count += 1;
                    prod *= num.val;
                    println!("{:?} {:?} {:?} {}", gear, num, check, prod);
                    break 'inner;
                }
            }
        }
        if count == 2 {
            sum += prod;
        }
    }
    println!("{}", sum);
}

#[allow(dead_code)]
fn part_one() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    // create a 2d array of boolean with one extra row and column around
    let mut adjacent = vec![vec![false; 300]; buffer.lines().count() + 3];
    for (y, line) in buffer.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !(c.is_numeric() || c == '.') {
                adjacent[y][x] = true;
                adjacent[y][x + 1] = true;
                adjacent[y][x + 2] = true;
                adjacent[y + 1][x] = true;
                adjacent[y + 1][x + 1] = true;
                adjacent[y + 1][x + 2] = true;
                adjacent[y + 2][x] = true;
                adjacent[y + 2][x + 1] = true;
                adjacent[y + 2][x + 2] = true;
            }
        }
    }
    for (y, line) in buffer.lines().enumerate() {
        let mut cur = 0;
        let mut is_adjacent = false;
        for (x, c) in line.chars().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    // numeric
                    cur = cur * 10 + c.to_digit(10).unwrap();
                    if adjacent[y + 1][x + 1] {
                        is_adjacent = true;
                    }
                    if x == line.len() - 1 {
                        if is_adjacent {
                            sum += cur;
                            is_adjacent = false;
                        }
                        cur = 0;
                    }
                }
                _ => {
                    // symbol
                    if is_adjacent {
                        sum += cur;
                        is_adjacent = false;
                    }
                    cur = 0;
                }
            }
        }
    }
    println!("{}", sum);
}
