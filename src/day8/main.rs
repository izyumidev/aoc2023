use regex::Regex;
use std::{collections::HashMap, io::Read};

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut iter = buffer.lines();
    let steps = iter.next().unwrap().chars().collect::<Vec<char>>();
    iter.next();
    let re = Regex::new(r"(\b[A-Z]{3}\b)").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut cur = "";
    for line in iter {
        let cap = re.captures_iter(line).collect::<Vec<_>>();
        if cur == "" {
            cur = cap[0].get(0).unwrap().as_str();
        }
        map.insert(
            cap[0].get(0).unwrap().as_str(),
            (
                cap[1].get(0).unwrap().as_str(),
                cap[2].get(0).unwrap().as_str(),
            ),
        );
    }
    let mut i = 0;
    while cur != "ZZZ" {
        let (l, r) = map.get(cur).unwrap();
        if steps[i % steps.len()] == 'L' {
            cur = l;
        } else {
            cur = r;
        }
        i += 1;
    }
    println!("{}", i);
}
