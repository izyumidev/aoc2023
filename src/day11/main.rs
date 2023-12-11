use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut empty_cols = vec![true; buffer.lines().next().unwrap().len()];
    let mut empty_rows = vec![true; buffer.lines().count()];
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row, line) in buffer.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                empty_cols[col] = false;
                empty_rows[row] = false;
                galaxies.push((row, col));
            }
        }
    }
    let mut empty_col_count = vec![0; empty_cols.len()];
    let mut empty_row_count = vec![0; empty_rows.len()];
    let mut cur = 0;
    for (i, row) in empty_rows.iter().enumerate() {
        if *row {
            cur += 1000000 - 1;
        }
        empty_row_count[i] = cur;
    }
    cur = 0;
    for (i, col) in empty_cols.iter().enumerate() {
        if *col {
            cur += 1000000 - 1;
        }
        empty_col_count[i] = cur;
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        let cur = galaxies[i];
        for j in (i + 1)..galaxies.len() {
            let to_compare = galaxies[j];
            let a = (cur.0 + empty_row_count[cur.0])
                .abs_diff(to_compare.0 + empty_row_count[to_compare.0]);
            let b = (cur.1 + empty_col_count[cur.1])
                .abs_diff(to_compare.1 + empty_col_count[to_compare.1]);
            // println!(
            //     "{} ({}, {}) {} ({}, {}) {}",
            //     i + 1,
            //     cur.0 + empty_row_count[cur.0],
            //     cur.1 + empty_col_count[cur.1],
            //     j + 1,
            //     to_compare.0 + empty_row_count[to_compare.0],
            //     to_compare.1 + empty_col_count[to_compare.1],
            //     a + b
            // );
            sum += a + b;
        }
    }
    println!("{}", sum);
}
