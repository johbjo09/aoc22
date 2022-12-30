use std::fs;
use std::cmp::{max,min};
use std::collections::VecDeque;
const GRIDSIZE: usize = 128;

struct BlizzardMap {
    ls: [[bool; GRIDSIZE]; GRIDSIZE],
    rs: [[bool; GRIDSIZE]; GRIDSIZE],
    ds: [[bool; GRIDSIZE]; GRIDSIZE],
    us: [[bool; GRIDSIZE]; GRIDSIZE],
    x_max: usize,
    y_max: usize,
}

impl BlizzardMap  {
    fn new() -> Self {
        Self {
            ls: [[false; GRIDSIZE]; GRIDSIZE],
            rs: [[false; GRIDSIZE]; GRIDSIZE],
            ds: [[false; GRIDSIZE]; GRIDSIZE],
            us: [[false; GRIDSIZE]; GRIDSIZE],
            x_max: 0,
            y_max: 0,
        }
    }

    fn blizz(&self, x: usize, y: usize, t: usize) -> bool {
        let tym = t % self.y_max;
        let txm = t % self.x_max;
        let y = y - 1;
        self.rs[y][(x + self.x_max - txm) % self.x_max] ||
            self.ds[(y + self.y_max - tym) % self.y_max][x] ||
            self.ls[y][(x + txm) % self.x_max] ||
            self.us[(y + tym) % self.y_max][x]
    }

    fn search(&self, start: (usize, usize, usize), x_end: usize, y_end: usize) -> usize {
        let mut t_min = start.2 + self.x_max * self.y_max;
        let mut visits = vec![vec![vec![]; self.x_max]; self.y_max+1];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while queue.len() > 0 {
            let (x,y,t) = queue.pop_front().unwrap();
            let nt = t + 1;
            let ix = x as i16;
            let iy = y as i16;
            for nt in nt..min(nt+6, t_min) {                
                for (dx,dy) in [(-1,0), (0,-1), (1,0), (0,1)] {
                    let nx = ix + dx;
                    let ny = iy + dy;
                    if nx>=0 && ny>=0 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if nx==x_end && ny==y_end && nt < t_min {
                            t_min = nt;
                        } else if nx<self.x_max && ny>0 && ny<=self.y_max {
                            if !self.blizz(nx, ny, nt) && !visits[ny][nx].contains(&nt) {
                                queue.push_back((nx,ny,nt));
                                visits[ny][nx].push(nt);
                            }
                        }
                    }
                }
                if y>0 {
                    if self.blizz(x, y, nt) {
                        break;
                    }
                }
            }
        }
        t_min
    }
}

fn solve(input: &str) -> (usize,usize) {
    let mut map = BlizzardMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if y > 0 && c != '#' {
                map.ls[y-1][x] = c == '<';
                map.rs[y-1][x] = c == '>';
                map.ds[y-1][x] = c == 'v';
                map.us[y-1][x] = c == '^';
                x += 1;
                map.x_max = max(x, map.x_max);
            }
        }
        map.y_max = max(y, map.y_max);
        y += 1;
    }
    let x_end = map.x_max - 1;
    let y_end = map.y_max;
    map.y_max -= 1;
    let t1 = map.search((0, 0, 0), x_end, y_end);
    let t2 = map.search((x_end, y_end, t1), 0, 0);
    let t3 = map.search((0, 0, t2), x_end, y_end);    
    (t1,t3)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 24: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
