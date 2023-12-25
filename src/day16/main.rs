use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    char: char,
    x: i32,
    y: i32,
    through: Vec<Direction>,
}

// part two
fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut map: Vec<Vec<Cell>> = Vec::new();
    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<Cell> = Vec::new();
        for c in line.chars() {
            row.push(Cell {
                char: c,
                x: 0,
                y: 0,
                through: Vec::new(),
            });
        }
        map.push(row);
    }
    let mut cur_max = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if y == 0 || y == map.len() - 1 || x == 0 || x == map[0].len() - 1 {
                for &direction in [Up, Down, Left, Right].iter() {
                    let num = get_num_of_powered(
                        &map,
                        Beam {
                            x: x as i32,
                            y: y as i32,
                            direction,
                        },
                    );
                    if num > cur_max {
                        cur_max = num;
                    }
                }
            }
        }
    }
    println!("{}", cur_max);
}

fn get_num_of_powered(map: &Vec<Vec<Cell>>, start_beam: Beam) -> i32 {
    let mut map = map.clone();
    let mut beams: Vec<Beam> = vec![start_beam];
    let mut powered: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    powered[start_beam.y as usize][start_beam.x as usize] = true;
    while !beams.is_empty() {
        let mut to_remove: Vec<Beam> = Vec::new();
        let mut new_beams: Vec<Beam> = Vec::new();
        for beam in beams.iter_mut() {
            let (mut next_x, mut next_y) = (beam.x, beam.y);
            match beam.direction {
                Up => next_y -= 1,
                Down => next_y += 1,
                Left => next_x -= 1,
                Right => next_x += 1,
            }
            if (next_x < 0 || next_x >= map[0].len() as i32)
                || (next_y < 0 || next_y >= map.len() as i32)
            {
                to_remove.push(*beam);
                continue;
            }
            let next = &map[next_y as usize][next_x as usize];
            let vertically_aligned =
                next.char == '|' && (beam.direction == Up || beam.direction == Down);
            let horizontally_aligned =
                next.char == '-' && (beam.direction == Left || beam.direction == Right);

            if vertically_aligned && next.through.contains(&beam.direction) {
                to_remove.push(*beam);
                continue;
            }

            if horizontally_aligned && next.through.contains(&beam.direction) {
                to_remove.push(*beam);
                continue;
            }

            if next.char == '|'
                && (beam.direction == Left || beam.direction == Right)
                && next.through.contains(&Up)
                && next.through.contains(&Down)
            {
                to_remove.push(*beam);
                continue;
            }

            if next.char == '-'
                && (beam.direction == Up || beam.direction == Down)
                && next.through.contains(&Left)
                && next.through.contains(&Right)
            {
                to_remove.push(*beam);
                continue;
            }

            if (vertically_aligned || horizontally_aligned)
                && powered[next_y as usize][next_x as usize]
            {
                to_remove.push(*beam);
                continue;
            }

            powered[next_y as usize][next_x as usize] = true;
            beam.x = next_x;
            beam.y = next_y;
            match next.char {
                '/' => match beam.direction {
                    Up => beam.direction = Right,
                    Down => beam.direction = Left,
                    Left => beam.direction = Down,
                    Right => beam.direction = Up,
                },
                '|' => {
                    if beam.direction == Left || beam.direction == Right {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Up;
                        new_beams.push(new_beam);
                        beam.direction = Down;
                        map[next_y as usize][next_x as usize].through.push(Up);
                        map[next_y as usize][next_x as usize].through.push(Down);
                    } else {
                        map[next_y as usize][next_x as usize]
                            .through
                            .push(beam.direction);
                    }
                }
                '\\' => match beam.direction {
                    Up => beam.direction = Left,
                    Down => beam.direction = Right,
                    Left => beam.direction = Up,
                    Right => beam.direction = Down,
                },
                '-' => {
                    if beam.direction == Up || beam.direction == Down {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Left;
                        new_beams.push(new_beam);
                        beam.direction = Right;
                        map[next_y as usize][next_x as usize].through.push(Left);
                        map[next_y as usize][next_x as usize].through.push(Right);
                    } else {
                        map[next_y as usize][next_x as usize]
                            .through
                            .push(beam.direction);
                    }
                }
                '.' => continue,
                _ => unreachable!("Invalid character"),
            }
        }
        beams.retain(|&beam| !to_remove.contains(&beam));
        beams.append(&mut new_beams);
        beams.dedup();
    }
    let mut count = 0;
    for cell in powered.iter() {
        for &is_powered in cell.iter() {
            if is_powered {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn part_one() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut map: Vec<Vec<Cell>> = Vec::new();
    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<Cell> = Vec::new();
        for c in line.chars() {
            row.push(Cell {
                char: c,
                x: 0,
                y: 0,
                through: Vec::new(),
            });
        }
        map.push(row);
    }
    let mut beams: Vec<Beam> = vec![Beam {
        x: -1,
        y: 0,
        direction: Right,
    }];
    let mut powered: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    powered[0][0] = true;
    while !beams.is_empty() {
        let mut to_remove: Vec<Beam> = Vec::new();
        let mut new_beams: Vec<Beam> = Vec::new();
        for beam in beams.iter_mut() {
            let (mut next_x, mut next_y) = (beam.x, beam.y);
            match beam.direction {
                Up => next_y -= 1,
                Down => next_y += 1,
                Left => next_x -= 1,
                Right => next_x += 1,
            }
            if (next_x < 0 || next_x >= map[0].len() as i32)
                || (next_y < 0 || next_y >= map.len() as i32)
            {
                to_remove.push(*beam);
                continue;
            }
            let next = &map[next_y as usize][next_x as usize];
            let vertically_aligned =
                next.char == '|' && (beam.direction == Up || beam.direction == Down);
            let horizontally_aligned =
                next.char == '-' && (beam.direction == Left || beam.direction == Right);

            if vertically_aligned && next.through.contains(&beam.direction) {
                to_remove.push(*beam);
                continue;
            }

            if horizontally_aligned && next.through.contains(&beam.direction) {
                to_remove.push(*beam);
                continue;
            }

            if next.char == '|'
                && (beam.direction == Left || beam.direction == Right)
                && next.through.contains(&Up)
                && next.through.contains(&Down)
            {
                to_remove.push(*beam);
                continue;
            }

            if next.char == '-'
                && (beam.direction == Up || beam.direction == Down)
                && next.through.contains(&Left)
                && next.through.contains(&Right)
            {
                to_remove.push(*beam);
                continue;
            }

            if (vertically_aligned || horizontally_aligned)
                && powered[next_y as usize][next_x as usize]
            {
                to_remove.push(*beam);
                continue;
            }

            powered[next_y as usize][next_x as usize] = true;
            beam.x = next_x;
            beam.y = next_y;
            match next.char {
                '/' => match beam.direction {
                    Up => beam.direction = Right,
                    Down => beam.direction = Left,
                    Left => beam.direction = Down,
                    Right => beam.direction = Up,
                },
                '|' => {
                    if beam.direction == Left || beam.direction == Right {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Up;
                        new_beams.push(new_beam);
                        beam.direction = Down;
                        map[next_y as usize][next_x as usize].through.push(Up);
                        map[next_y as usize][next_x as usize].through.push(Down);
                    } else {
                        map[next_y as usize][next_x as usize]
                            .through
                            .push(beam.direction);
                    }
                }
                '\\' => match beam.direction {
                    Up => beam.direction = Left,
                    Down => beam.direction = Right,
                    Left => beam.direction = Up,
                    Right => beam.direction = Down,
                },
                '-' => {
                    if beam.direction == Up || beam.direction == Down {
                        let mut new_beam = beam.clone();
                        new_beam.direction = Left;
                        new_beams.push(new_beam);
                        beam.direction = Right;
                        map[next_y as usize][next_x as usize].through.push(Left);
                        map[next_y as usize][next_x as usize].through.push(Right);
                    } else {
                        map[next_y as usize][next_x as usize]
                            .through
                            .push(beam.direction);
                    }
                }
                '.' => continue,
                _ => unreachable!("Invalid character"),
            }
        }
        beams.retain(|&beam| !to_remove.contains(&beam));
        beams.append(&mut new_beams);
        beams.dedup();
    }
    let mut count = 0;
    for cell in powered.iter() {
        for &is_powered in cell.iter() {
            if is_powered {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
