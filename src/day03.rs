use std::fs;

fn solve1(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;
    
    'line: for line in lines {
        if line.len() % 2 == 0 {
            let h = line.len() / 2;
            for i in 0..h {
                let c0 = line.chars().nth(i).unwrap();
                if c0 >= 'A' {
                    for j in h..line.len() {
                        if c0 == line.chars().nth(j).unwrap() {
                            let prio = (c0 as u32) - if c0 < 'a' { 38 } else { 96 };
                            sum += prio;
                            continue 'line;
                        }
                    }
                }
            }
        } else {
            println!("Not even: {}", line);
        }
    }
    sum
}

fn solve2(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;
    let mut badge: [[bool;3]; 58] = [[false;3]; 58];
    let mut count = 0;
    
    for line in lines {
        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c >= 'A' {
                let p = (c as usize) - if c < 'a' { 38 } else { 96 };
                badge[p][count] = true;
            }
        }
        if count == 2 {
            for i in 0..badge.len() {
                if badge[i] == [true,true,true] {
                    sum += i as u32;
                }
            }
            badge = [[false;3]; 58];
            count = 0;
        } else {
            count += 1;
        }
    }
    sum
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let lines = contents.lines().collect();
            println!("Day 03: {} {}", solve1(&lines), solve2(&lines));
        },
        Err(err) => println!("{}", err),
    }
}
