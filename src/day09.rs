use std::fs;

fn solve(input: &str) -> (usize,usize) {
    let mut rope = [[0_i32; 2]; 10];
    let mut path1: Vec<(i32,i32)> = Vec::with_capacity(8192);
    let mut path2: Vec<(i32,i32)> = Vec::with_capacity(4096);
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let steps = parts[1].parse::<usize>().unwrap();
        let headmove = match parts[0] {
            "R" => (1,0),
            "U" => (0,1),
            "D" => (0,-1),
            "L" => (-1,0),
            _ => (0,0),
        };

        for _ in 0..steps {
            rope[0][0] += headmove.0;
            rope[0][1] += headmove.1;
            
            for i in 1..10 {
                for _ in 0..2 {
                    let diff = (rope[i-1][0] - rope[i][0], rope[i-1][1] - rope[i][1]);
                    if diff.0.abs()==1 && diff.0.abs()<diff.1.abs() {
                        rope[i][0] += diff.0.signum();
                    } else if diff.1.abs()==1 && diff.0.abs()>diff.1.abs() {
                        rope[i][1] += diff.1.signum();
                    } else {
                        if diff.0.abs()>1 { rope[i][0] += diff.0.signum(); }
                        if diff.1.abs()>1 { rope[i][1] += diff.1.signum(); }
                    }
                }
            }
            let tail1 = (rope[1][0],rope[1][1]);
            let tail2 = (rope[9][0],rope[9][1]);
            if !path1.contains(&tail1) { path1.push(tail1); }
            if !path2.contains(&tail2) { path2.push(tail2); }
        }
    }
    
    (path1.len(), path2.len())
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 09: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
