use std::fs;
use std::cmp::max;
use std::collections::VecDeque;

fn solve(input: &str) -> (usize,usize) {
    let mut p = vec![];
    let mut sides1 = 0;
    
    for line in input.lines() {
        let x: Vec<usize> = line.trim().split(",").map(|s| s.parse().unwrap()).collect();
        p.push([x[0] + 1, x[1] + 1, x[2] + 1]);
        sides1 += 6;
    }

    let n = p.len();

    let mut pmax = 0;
    
    for i in 0..n {
        for k in 0..3 {
            pmax = max(pmax, p[i][k]);
        }        
        for j in i+1..n {
            if p[i][0] == p[j][0] && p[i][1] == p[j][1] && (p[i][2] as i32 - p[j][2] as i32).abs() == 1 {
                sides1 -= 2;
            }
            if p[i][1] == p[j][1] && p[i][2] == p[j][2] && (p[i][0] as i32 - p[j][0] as i32).abs() == 1 {
                sides1 -= 2;
            }
            if p[i][0] == p[j][0] && p[i][2] == p[j][2] && (p[i][1] as i32 - p[j][1] as i32).abs() == 1 {
                sides1 -= 2;
            }
        }
    }

    pmax += 2;
    
    let mut ps = vec![vec![vec![false; pmax]; pmax]; pmax];
    let mut visited = vec![vec![vec![false; pmax]; pmax]; pmax];
    for i in 0..n {
        ps[p[i][0]][p[i][1]][p[i][2]] = true;
    }

    let mut sides2 = 0;
    let mut queue: VecDeque<(usize,usize,usize)> = VecDeque::new();    
    queue.push_back((0,0,0));
    
    while queue.len() > 0 {
        let (x,y,z) = queue.pop_front().unwrap();        
        for (dx,dy,dz) in [(1,0,0), (0,1,0), (0,0,1), (-1,0,0), (0,-1,0), (0,0,-1)] {
            if x as i32 + dx >= 0 && y as i32 + dy >= 0 && z as i32 + dz >= 0 {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                let nz = (z as i32 + dz) as usize;
                if nx < pmax && ny < pmax && nz < pmax {
                    if !visited[nx][ny][nz] {
                        if ps[nx][ny][nz] {
                            sides2 += 1;
                        } else {
                            visited[nx][ny][nz] = true;
                            queue.push_back((nx,ny,nz));
                        }
                    }
                }
            }
        }
    }    
    (sides1, sides2)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 18: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
