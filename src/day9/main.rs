use std::{
    io::{self, Read},
    vec,
};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in buffer.lines() {
        let mut row = Vec::new();
        line.split_whitespace().for_each(|x| {
            let number = x.parse::<i32>().unwrap();
            row.push(number);
        });
        numbers.push(row);
    }
    for row in numbers.iter() {
        let mut seq: Vec<Vec<i32>> = vec![row.clone()];
        loop {
            let mut cur_seq: Vec<i32> = Vec::new();
            let mut last_num = 0;
            let last = seq.last().unwrap();
            if last.iter().all(|&x| x == 0) {
                break;
            }
            for num in last.iter() {
                if last_num == 0 {
                    last_num = *num;
                    continue;
                }
                cur_seq.push(*num - last_num);
                last_num = *num;
            }
            if cur_seq.is_empty() {
                break;
            }
            seq.push(cur_seq);
        }
        let mut cur_sum = 0;
        seq.iter()
            .for_each(|s| cur_sum += s.last().unwrap().to_owned());
        sum += cur_sum;
    }
    println!("{}", sum);
}
