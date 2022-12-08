use std::fs;

fn solve(input: &str) -> (usize, usize) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut grid_transpose: Vec<Vec<u8>> = Vec::new();
    let mut visibility: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<usize>> = Vec::new();
    
    for line in input.lines() {
        let y = grid.len();
        grid.push(Vec::new());
        visibility.push(Vec::new());
        distance.push(Vec::new());
        for c in line.chars() {
            grid[y].push((c as u8) - ('0' as u8));
            visibility[y].push(false);
            distance[y].push(1);
        }
    }

    let szy = grid.len();
    let szx = grid[0].len();
    
    for x in 0..szx {
        grid_transpose.push(Vec::new());
        for y in 0..szy {
            grid_transpose[x].push(grid[y][x]);
        }
    }

    fn compute(h: u8, row1: &[u8], row2: &[u8]) -> (usize, bool) {
        let visible = row1.iter().all(|&d| d<h) || row2.iter().rev().all(|&d| d<h);
        let mut dist1 = 0;
        for &d in row1.into_iter().rev() {
            dist1 += 1;
            if h<=d {
                break;
            }
        }
        let mut dist2 = 0;
        for &d in row2.into_iter() {
            dist2 += 1;
            if h<=d {
                break;
            }
        }
        (dist1 * dist2, visible)
    }

    for y in 1..szy-1 {
        for x in 1..szx-1 {
            let (dist, v) = compute(grid[y][x], &grid[y][0..x], &grid[y][x+1..szx]);
            visibility[y][x] = visibility[y][x] || v;
            distance[y][x] *= dist;
        }
    }

    for x in 1..szx-1 {
        for y in 1..szy-1 {
            let (dist, v) = compute(grid[y][x], &grid_transpose[x][0..y], &grid_transpose[x][y+1..szy]);
            visibility[y][x] = visibility[y][x] || v;
            distance[y][x] *= dist;
        }
    }

    let mut count = 2*szx + 2*szy - 4;
    let mut dmax = 0;
    
    for y in 1..szy-1 {
        for x in 1..szx-1 {
            if visibility[y][x] {
                count += 1;
            }
            if dmax < distance[x][y] {
                dmax = distance[x][y];
            }
        }
    }

    (count, dmax)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 08: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
