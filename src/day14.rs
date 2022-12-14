use std::fs;
use std::cmp::{min, max};

fn solve(input: &str) -> u32 {
    let mut map = [[' '; 1024]; 1024];
    let mut floor = 0;
    
    for line in input.lines() {
        let coords: Vec<Vec<usize>> = line.split(" -> ")
            .map(|s| s.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect();
        for i in 1..coords.len() {
            let x0 = min(coords[i-1][0], coords[i][0]);
            let x1 = max(coords[i-1][0], coords[i][0]);
            let y0 = min(coords[i-1][1], coords[i][1]);
            let y1 = max(coords[i-1][1], coords[i][1]);

            floor = max(floor, y1);
            
            for x in x0..x1+1 {
                for y in y0..y1+1 {
                    map[y][x] = '#';
                }
            }
        }
    }

    floor += 1;

    let mut count = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            let mut nx = x;
            let ny = y + 1;
            if map[ny][nx] != ' ' {
                nx = x - 1;
                if map[ny][nx] != ' ' {
                    nx = x + 1;
                    if map[ny][nx] != ' ' {
                        map[y][x] = 'o';
                        count += 1;
                        break;
                    }
                }
            }

            x = nx;
            y = ny;
            
            if y==floor {
                map[y][x] = 'o';
                count += 1;
                break;
            }
        }

        if x==500 && y==0 {
            break;
        }
    }
    
    count
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 14: {}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
