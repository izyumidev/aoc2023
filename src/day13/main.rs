use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut rows: Vec<u32> = vec![0; 30];
    let mut cols: Vec<u32> = vec![0; 30];
    let mut sum = 0;
    let mut cur_starts_at = 0;
    for (y, line) in buffer.lines().enumerate() {
        if line.is_empty() {
            rows = vec![0; 30];
            cols = vec![0; 30];
            cur_starts_at = y;
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rows[y - cur_starts_at] |= 1 << x;
                    cols[x] |= 1 << (y - cur_starts_at);
                }
                '.' => (),
                _ => panic!("Invalid character"),
            }
        }
    }
    let mut prev_row = &0u32;
    let mut reflects_vertically = false;
    for (i, row) in rows.iter().enumerate() {
        println!("{i} {:b} {:b}", prev_row, row);
        if row & prev_row != *row {
            prev_row = row;
        } else {
            println!("{i}");
            sum += i;
            reflects_vertically = true;
            break;
        }
    }
    if !reflects_vertically {
        let mut prev_col = &0u32;
        for (i, col) in cols.iter().enumerate() {
            if col & prev_col != *col {
                prev_col = col;
            } else {
                sum += 100 * i;
                break;
            }
        }
    }
    println!("{sum}");
}
