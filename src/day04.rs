use std::fs;
use regex::Regex;

fn solve(contents: &str) -> (u32, u32) {
    let mut count = (0,0);
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut n = [0, 0, 0, 0];
    for line in contents.lines() {
        let l = re.captures(line).unwrap();
        for i in 0..4 {
            n[i] = l[i+1].parse::<u16>().unwrap();
        }
        count.0 += ((n[0] <= n[2] && n[3] <= n[1]) || (n[2] <= n[0] && n[1] <= n[3])) as u32;
        count.1 += (n[0] <= n[3] && (n[2] <= n[1] || n[2] <= n[0])) as u32;
    }
    count
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 04: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
