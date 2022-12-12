use std::fs;
use std::collections::VecDeque;

fn solve(input: &str, prob2: bool) -> i16 {
    let mut grid: Vec<Vec<i16>> = vec![];
    let mut distgrid: Vec<Vec<i16>> = vec![];
    let mut end = [0,0];

    let mut queue = VecDeque::new();
    
    for line in input.lines() {
        let y = grid.len();
        grid.push(vec![]);
        distgrid.push(vec![]);
        let mut x = 0;
        for c in line.chars() {
            let mut v = c as i16;
            if c == 'S' {
                queue.push_back((0, x, y));
                v = 'a' as i16;
            } else if c == 'E' {
                end = [x, y];
                v = 'z' as i16;
            }
            grid[y].push(v);
            distgrid[y].push(std::i16::MAX);
            x += 1;
        }
    }

    let szy = grid.len();
    let szx = grid[0].len();
    let mut mindist = std::i16::MAX;

    while queue.len() > 0 {
        let (dist,x,y) = queue.pop_front().unwrap();
        if dist <= distgrid[y][x] {
            for dx in [-1_i16, 0_i16, 1_i16] {
                for dy in [-1_i16, 0_i16, 1_i16] {
                    if dx.abs()!=dy.abs() {
                        let nx = x as i16 + dx;
                        let ny = y as i16 + dy;
                        if nx>=0 && ny>=0 {
                            let nx = nx as usize;
                            let ny = ny as usize;                            
                            if nx<szx && ny<szy {
                                let step = grid[ny][nx] - grid[y][x];
                                if step < 2 {
                                    let nextdist = if prob2 && grid[y][x]=='a' as i16 { 1 } else { dist + 1 };
                                    if [nx,ny]==end && nextdist<mindist {
                                        mindist = nextdist;
                                    } else {
                                        if nextdist < distgrid[ny][nx] {
                                            distgrid[ny][nx] = nextdist;
                                            if [nx,ny]==end && nextdist<mindist {
                                                mindist = nextdist;
                                            } else  {
                                                queue.push_back((nextdist,nx,ny));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    mindist
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 12: {} {}", solve(&contents, false), solve(&contents, true)),
        Err(err) => println!("{}", err),
    }
}
