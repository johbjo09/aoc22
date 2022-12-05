use std::fs;
use std::collections::VecDeque;

fn solve(input: &str, prob1: bool) -> String {
    let mut stacks: Vec<VecDeque<char>> = Default::default();

    for line in input.lines() {
        if line.chars().any(|c| c == '[') {
            let mut p = 0;
            let mut i = 0;
            for c in line.chars() {
                if ((p-1) % 4) == 0 {
                    if stacks.len() < i+1 {
                        stacks.push(VecDeque::new());
                    }
                    if c != ' ' {
                        stacks[i].push_back(c);
                    }
                    i += 1;
                }
                p += 1;
            }
        } else if line.contains("move") {
            let l: Vec<&str> = line.split(' ').collect();
            let count = l[1].parse::<usize>().unwrap();
            let from = l[3].parse::<usize>().unwrap() - 1;
            let to = l[5].parse::<usize>().unwrap() - 1;
            if prob1 {
                for _ in 0..count {
                    let c = stacks[from].pop_front().unwrap();
                    stacks[to].push_front(c);
                }
            } else {
                let mut moves: Vec<char> = Vec::with_capacity(count);
                for _ in 0..count {
                    let c = stacks[from].pop_front().unwrap();
                    moves.push(c);
                }
                for i in 0..count {
                    stacks[to].push_front(moves[count-i-1]);
                }
            }
        }
    }

    stacks.iter().map(|s| s[0]).collect()
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 05: {} {}", solve(&contents, true), solve(&contents, false)),
        Err(err) => println!("{}", err),
    }
}
