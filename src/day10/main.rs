use std::io::{self, Read};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut cells: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();
    let mut path_len = 0;
    // find S
    let mut cur_pos: (usize, usize) = (0, 0);
    for (i, row) in cells.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                cur_pos = (i, j);
            }
        }
    }
    // check initial direction
    let mut cur_dir = Direction::Up;
    if cur_pos.0 != 0
        && (cells[cur_pos.0 - 1][cur_pos.1] == '|'
            || cells[cur_pos.0 - 1][cur_pos.1] == '7'
            || cells[cur_pos.0 - 1][cur_pos.1] == 'F')
    {
        cur_dir = Direction::Up;
    } else if cur_pos.0 != cells.len() - 1
        && (cells[cur_pos.0 + 1][cur_pos.1] == '|'
            || cells[cur_pos.0 + 1][cur_pos.1] == 'J'
            || cells[cur_pos.0 + 1][cur_pos.1] == 'L')
    {
        cur_dir = Direction::Down;
    } else if cur_pos.1 != 0
        && (cells[cur_pos.0][cur_pos.1 - 1] == '-'
            || cells[cur_pos.0][cur_pos.1 - 1] == 'F'
            || cells[cur_pos.0][cur_pos.1 - 1] == 'L')
    {
        cur_dir = Direction::Left;
    } else if cur_pos.1 != cells[0].len() - 1
        && (cells[cur_pos.0][cur_pos.1 + 1] == '-'
            || cells[cur_pos.0][cur_pos.1 + 1] == '7'
            || cells[cur_pos.0][cur_pos.1 + 1] == 'J')
    {
        cur_dir = Direction::Right;
    }
    // loop
    loop {
        path_len += 1;
        match cur_dir {
            Direction::Up => {
                if cells[cur_pos.0 - 1][cur_pos.1] == '|' {
                    cur_pos.0 -= 1;
                } else if cells[cur_pos.0 - 1][cur_pos.1] == '7' {
                    cur_pos.0 -= 1;
                    cur_dir = Direction::Left;
                } else if cells[cur_pos.0 - 1][cur_pos.1] == 'F' {
                    cur_pos.0 -= 1;
                    cur_dir = Direction::Right;
                } else {
                    break;
                }
            }
            Direction::Down => {
                if cells[cur_pos.0 + 1][cur_pos.1] == '|' {
                    cur_pos.0 += 1;
                } else if cells[cur_pos.0 + 1][cur_pos.1] == 'J' {
                    cur_pos.0 += 1;
                    cur_dir = Direction::Left;
                } else if cells[cur_pos.0 + 1][cur_pos.1] == 'L' {
                    cur_pos.0 += 1;
                    cur_dir = Direction::Right;
                } else {
                    break;
                }
            }
            Direction::Left => {
                if cells[cur_pos.0][cur_pos.1 - 1] == '-' {
                    cur_pos.1 -= 1;
                } else if cells[cur_pos.0][cur_pos.1 - 1] == 'F' {
                    cur_pos.1 -= 1;
                    cur_dir = Direction::Up;
                } else if cells[cur_pos.0][cur_pos.1 - 1] == 'L' {
                    cur_pos.1 -= 1;
                    cur_dir = Direction::Down;
                } else {
                    break;
                }
            }
            Direction::Right => {
                if cells[cur_pos.0][cur_pos.1 + 1] == '-' {
                    cur_pos.1 += 1;
                } else if cells[cur_pos.0][cur_pos.1 + 1] == '7' {
                    cur_pos.1 += 1;
                    cur_dir = Direction::Up;
                } else if cells[cur_pos.0][cur_pos.1 + 1] == 'J' {
                    cur_pos.1 += 1;
                    cur_dir = Direction::Down;
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", (path_len + 1) / 2);
}
