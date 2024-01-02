use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut maps: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut cur_map: Vec<Vec<bool>> = Vec::new();
    for line in buffer.lines() {
        if line.is_empty() {
            maps.push(cur_map.clone());
            continue;
        }
        let mut cur_row: Vec<bool> = Vec::new();
        for c in line.chars() {
            match c {
                '#' => cur_row.push(true),
                '.' => cur_row.push(false),
                _ => unreachable!(),
            }
        }
        cur_map.push(cur_row);
    }
    if !cur_map.is_empty() {
        maps.push(cur_map);
    }
    let mut sum = 0;
    'map_loop: for map in maps.iter() {
        let mut prev_row = None;
        for (y, row) in map.iter().enumerate() {
            if prev_row.eq(&Some(row)) {
                println!("{} {:?}", y, row);
                sum += (y + 1) * 100;
                continue 'map_loop;
            }
            prev_row = Some(row);
        }
        let col_num = map.iter().next().unwrap().len();
        for x in 0..col_num {
            let mut prev_col = None;
            for y in 0..map.len() {
                if prev_col.eq(&Some(map[y][x])) {
                    sum += x + 1;
                    continue 'map_loop;
                }
                prev_col = Some(map[y][x]);
            }
        }
    }
    println!("{}", sum);
}
