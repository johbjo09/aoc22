use std::fs;

fn solve(input: &str, l: usize) -> usize {
    let mut buf: Vec<char> = Vec::with_capacity(l);
    for _ in 0..l {
        buf.push('#');
    }
    let mut i = 0;
    'charLoop: for ch in input.chars() {
        buf[i % l] = ch;
        i += 1;
        if i > l {
            for j in 0..l {
                for k in j+1..l {
                    if buf[j] == buf[k] {
                        continue 'charLoop;
                    }
                }
            }
            return i;
        }
    }
    i
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 06: {} {}", solve(&contents, 4), solve(&contents, 14)),
        Err(err) => println!("{}", err),
    }
}
