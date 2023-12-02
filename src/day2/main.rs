use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut sum = 0;
    buffer.lines().for_each(|line| {
        let a = line.split(':').collect::<Vec<&str>>();
        let n = a[0].split(' ').last().unwrap().parse::<usize>().unwrap();
        let mut count = vec![0; 3];
        a[1].split(';').for_each(|b| {
            b.trim()
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|d| {
                    let c = d.trim().split(' ').collect::<Vec<&str>>();
                    match c[1] {
                        "red" => {
                            if count[0] < c[0].parse::<usize>().unwrap() {
                                count[0] = c[0].parse::<usize>().unwrap();
                            }
                        }
                        "green" => {
                            if count[1] < c[0].parse::<usize>().unwrap() {
                                count[1] = c[0].parse::<usize>().unwrap();
                            }
                        }
                        "blue" => {
                            if count[2] < c[0].parse::<usize>().unwrap() {
                                count[2] = c[0].parse::<usize>().unwrap();
                            }
                        }
                        _ => {}
                    }
                })
        });
        sum += count[0] * count[1] * count[2];
    });
    println!("{}", sum);
}
