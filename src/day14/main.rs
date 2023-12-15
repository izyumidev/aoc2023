use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if x == 0 {
                grid.push(Vec::new());
            }
            grid[y].push(c);
        }
    }
    for _ in 0..1000000000 {
        for _ in 0..4 {
            rotate(&mut grid);
            tilt_grid(&mut grid);
        }
    }
    let mut sum = 0;
    for line in grid.iter() {
        for (i, c) in line.iter().enumerate() {
            if *c == 'O' {
                sum += i + 1;
            }
        }
    }
    println!("{}", sum);
}

fn rotate(grid: &mut Vec<Vec<char>>) {
    let mut rotated_grid: Vec<Vec<char>> = Vec::new();
    for x in 0..grid[0].len() {
        rotated_grid.push(Vec::new());
        for y in (0..grid.len()).rev() {
            rotated_grid[x].push(grid[y][x]);
        }
    }
    *grid = rotated_grid;
}

fn tilt_grid(grid: &mut Vec<Vec<char>>) {
    let mut tilted_grid: Vec<Vec<char>> = Vec::new();
    for line in grid.iter_mut() {
        let mut cur_round_count = 0;
        let mut cur_count = 0;
        let mut updated_line: Vec<char> = Vec::new();
        'char_loop: for c in line.iter() {
            match c {
                'O' => cur_round_count += 1,
                '#' => {
                    let ground_count = cur_count - cur_round_count;
                    for _ in 0..ground_count {
                        updated_line.push('.');
                    }
                    for _ in 0..cur_round_count {
                        updated_line.push('O');
                    }
                    cur_round_count = 0;
                    cur_count = 0;
                    updated_line.push('#');
                    continue 'char_loop;
                }
                '.' => (),
                _ => unreachable!("Invalid char"),
            }
            cur_count += 1;
        }
        if cur_count > 0 {
            let ground_count = cur_count - cur_round_count;
            for _ in 0..ground_count {
                updated_line.push('.');
            }
            for _ in 0..cur_round_count {
                updated_line.push('O');
            }
        }
        tilted_grid.push(updated_line);
    }
    *grid = tilted_grid;
}

#[allow(dead_code)]
fn part_one() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if x == 0 {
                grid.push(Vec::new());
            }
            grid[y].push(c);
        }
    }
    let mut rotated_grid: Vec<Vec<char>> = Vec::new();
    for x in 0..grid[0].len() {
        rotated_grid.push(Vec::new());
        for y in (0..grid.len()).rev() {
            rotated_grid[x].push(grid[y][x]);
        }
    }
    let mut tilted_grid: Vec<Vec<char>> = Vec::new();
    for line in rotated_grid.iter_mut() {
        let mut cur_round_count = 0;
        let mut cur_count = 0;
        let mut updated_line: Vec<char> = Vec::new();
        'char_loop: for c in line.iter() {
            match c {
                'O' => cur_round_count += 1,
                '#' => {
                    let ground_count = cur_count - cur_round_count;
                    for _ in 0..ground_count {
                        updated_line.push('.');
                    }
                    for _ in 0..cur_round_count {
                        updated_line.push('O');
                    }
                    cur_round_count = 0;
                    cur_count = 0;
                    updated_line.push('#');
                    continue 'char_loop;
                }
                '.' => (),
                _ => unreachable!("Invalid char"),
            }
            cur_count += 1;
        }
        if cur_count > 0 {
            let ground_count = cur_count - cur_round_count;
            for _ in 0..ground_count {
                updated_line.push('.');
            }
            for _ in 0..cur_round_count {
                updated_line.push('O');
            }
        }
        tilted_grid.push(updated_line);
    }
    let mut sum = 0;
    for line in tilted_grid.iter() {
        for (i, c) in line.iter().enumerate() {
            if *c == 'O' {
                sum += i + 1;
            }
        }
    }
    println!("{}", sum);
}
