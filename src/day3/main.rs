use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

struct Number {
    val: u32,
    points: Vec<Point>,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        let mut cur = 0;
        let mut points = vec![];
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                cur = cur * 10 + c.to_digit(10).unwrap();
                points.push(Point {
                    x: x as u32,
                    y: y as u32,
                });
            } else {
                if cur != 0 {
                    numbers.push(Number {
                        val: cur,
                        points: points.clone(),
                    });
                    cur = 0;
                }
            }
        }
    }
    for (y, line) in buffer.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let mut adjacent_numbers: Vec<u32> = Vec::new();
                for num in &numbers {
                    for point in &num.points {
                        if point.x.abs_diff(x as u32) <= 1 && point.y.abs_diff(y as u32) <= 1 {
                            adjacent_numbers.push(num.val);
                            break;
                        }
                    }
                }
                if adjacent_numbers.len() == 2 {
                    println!("{:?}", adjacent_numbers);
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }
    println!("{sum}")
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
