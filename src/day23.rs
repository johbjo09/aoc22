use std::fs;
use std::cmp::{max,min};

const GRIDSIZE: usize = 256;

const DIRECTIONS: [[(i16,i16); 3]; 4] = [[ (-1,-1), ( 0,-1), ( 1,-1) ],
                                         [ (-1, 1), ( 0, 1), ( 1, 1) ],
                                         [ (-1,-1), (-1, 0), (-1, 1) ],
                                         [ ( 1,-1), ( 1, 0), ( 1, 1) ]];

const NEIGHBOURS: [(i16,i16); 8] = [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)];

fn solve(input: &str) -> (usize,usize) {
    let mut elves = vec![];
    let mut grid = vec![[false; GRIDSIZE]; GRIDSIZE];
    let mut proposalgrid = vec![[-1_i16; GRIDSIZE]; GRIDSIZE];
    let mut proposals = vec![];
    let mut min_y = GRIDSIZE-1;
    let mut min_x = GRIDSIZE-1;
    let mut max_y = 0;
    let mut max_x = 0;
    {
        let mut y = GRIDSIZE/2;
        for line in input.lines() {
            let mut x = GRIDSIZE/2;
            for c in line.chars() {
                if c=='#' {
                    grid[y][x] = true;
                    elves.push((x as i16, y as i16));
                    proposals.push((0,0));
                    min_y = min(y, min_y);
                    min_x = min(x, min_x);
                    max_x = max(x, max_x);
                    max_y = max(y, max_y);
                }
                x += 1;
            }
            y += 1;
        }
    }
    
    let mut freecount = 0;
    let mut rounds = 0;

    loop {
        let mut k = 0;
        for i in 0..elves.len() {
            let (x,y) = elves[i];
            let mut alone = true;
            for (dx,dy) in NEIGHBOURS {
                let tx = (x + dx) as usize;
                let ty = (y + dy) as usize;
                if grid[ty][tx] {
                    alone = false;
                    break;
                }
            }
            if !alone {
                for j in 0..4 {
                    let dir = (j + rounds) % 4;
                    let mut empty = true;
                    for (dx,dy) in DIRECTIONS[dir] {
                        let tx = (x + dx) as usize;
                        let ty = (y + dy) as usize;
                        if grid[ty][tx] {
                            empty = false;
                        }
                    }
                    if empty {
                        let (dx,dy) = DIRECTIONS[dir][1];
                        let tx = (x + dx) as usize;
                        let ty = (y + dy) as usize;
                        let p = proposalgrid[ty][tx];
                        if p == -1 {
                            proposals[k] = (tx, ty);
                            k += 1;
                            proposalgrid[ty][tx] = i as i16;
                        } else if p > -1 {
                            proposalgrid[ty][tx] = -2;
                        }
                        break;
                    }
                }
            }
        }
        
        let mut moves = false;
        for j in 0..k {
            let (x,y) = proposals[j];
            let p = proposalgrid[y][x];
            if p > -1 {
                moves = true;
                let i = p as usize;
                let (ox,oy) = elves[i];
                grid[oy as usize][ox as usize] = false;
                grid[y][x] = true;
                elves[i] = (x as i16, y as i16);
                min_y = min(y, min_y);
                max_y = max(y, max_y);
                min_x = min(x, min_x);
                max_x = max(x, max_x);
            }
            proposalgrid[y][x] = -1;
        }

        rounds += 1;
        
        if !moves {
            break;
        }

        if rounds == 10 {
            for y in min_y..max_y+1 {
                for x in min_x..max_x+1 {
                    if !grid[y][x] {
                        freecount += 1;
                    }
                }
            }
        }
    }
    
    (freecount,rounds)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 23: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
