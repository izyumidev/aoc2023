use std::io::{self, Read};

fn main() {
    let mut butffer = String::new();
    io::stdin().read_to_string(&mut butffer).unwrap();
    let mut sum = 0;
    for lines in butffer.lines() {
        let mut line = lines.split_whitespace();
        let left = line.next().unwrap().to_string();
        let right = line
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let left_as_char_vec = left.chars().collect::<Vec<char>>();
        let num_of_questions = left_as_char_vec.iter().filter(|&x| *x == '?').count();
        for i in 0..(1 << num_of_questions) {
            let mut tmp: Vec<char> = Vec::new();
            let mut j = 0;
            for c in left_as_char_vec.iter() {
                if *c == '?' {
                    if (i >> j) & 1 == 1 {
                        tmp.push('#');
                    } else {
                        tmp.push('.');
                    }
                    j += 1;
                } else {
                    tmp.push(*c);
                }
            }
            let tmp_str = tmp.iter().collect::<String>();
            if parse(tmp_str) == right {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

fn parse(str: String) -> Vec<i32> {
    let mut res = Vec::new();
    let mut cur = 0;
    for c in str.chars() {
        if c == '#' {
            cur += 1;
        } else if cur != 0 {
            res.push(cur);
            cur = 0;
        }
    }
    if cur != 0 {
        res.push(cur);
    }
    res
}
