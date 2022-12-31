use std::fs;

fn toint(snafu: &str) -> i64 {
    let mut nr = 0;
    let mut p = 1;
    for c in snafu.chars().rev() {
        nr += p * match c { '2' => 2,
                             '1' => 1,
                             '-' => -1,
                             '=' => -2,
                             _ => 0,
        };
        p *= 5;
    }
    nr
}

fn tosnafu(mut nr: i64) -> String {
    let mut snafu = String::new();
    let mut rem = 0;
    while nr > 0 || rem > 0 {
        let mut s = rem + nr % 5;
        nr = nr / 5;
        if s < 3 {
            rem = 0;
        } else {
            s -= 5;
            rem = 1;
        }
        snafu.push_str(match s { 1 => "1",
                                 2 => "2",
                                 -1 => "-",
                                 -2 => "=",
                                 _ => "0" });
    }
    snafu.chars().rev().collect()
}

fn solve(input: &str) -> String {
    let sum = input.lines().map(|l| toint(l)).sum();
    tosnafu(sum)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 25: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
