use std::fs;
use std::cmp::{min,max};

fn solve(input: &str) -> (usize,usize) {
    let winds: Vec<bool> = input.trim().chars().map(|c| c=='<').collect();
    let mut room: Vec<[bool; 7]> = vec![];
    let mut hashes = [0; 4096];
    let mut rowcounts = [0; 4096];
    let mut toprow = 0;
    let mut towerheight = 0;
    let mut i = 0;
    let mut j = 0;
    let mut result1 = 0;
    
    loop {    
        let mut rowhash = 0;
        for r in 0..min(8, toprow) {
            rowhash ^= (0..7).fold(0, |a,k| (a << 1) | (room[toprow - r][k] as usize));
        }
        let hash = ((j << 3 | i%5) << 7) | rowhash;
        
        if i < hashes.len() {
            hashes[i] = hash;
            rowcounts[i] = towerheight;
        }

        if i%5 == 0 {
            let is: Vec<usize> = (0..min(i, hashes.len())).filter(|&k| hash == hashes[k]).collect();
            if let Some(&lowi) = is.iter().min() {
                if let Some(&highi) = is.iter().max() {
                    if lowi < highi {
                        let ijump = 100 * (highi - lowi);
                        let towerinc = 100 * (rowcounts[highi] - rowcounts[lowi]);
                        while i + ijump < 1000000000000 {
                            i += ijump;
                            towerheight += towerinc;
                        }
                    }
                }
            }
        }
        
        let (rockheight, rock) = match i % 5 {
            0 => (1, [true , true , true , true , false, false, false, false, false, false, false, false, false, false, false, false]),
            1 => (3, [false, true , false, false, true , true , true , false, false, true , false, false, false, false, false, false]),
            2 => (3, [false, false, true , false, false, false, true , false, true , true , true , false, false, false, false, false]),
            3 => (4, [true , false, false, false, true , false, false, false, true , false, false, false, true , false, false, false]),
            _ => (2, [true , true , false, false, true , true , false, false, false, false, false, false, false, false, false, false]),
        };
        let need = rockheight + 3;
        let mut freerows = room.len() - toprow;
        if freerows < need {
            for _ in 0..need-freerows {
                room.push([false, false, false, false, false, false, false]);
            }
            freerows = need;
        }

        let mut y = room.len() - freerows + need - 1;
        let mut x = 2;

        'falloop: loop {
            let windleft = winds[j];
            j = (j+1) % winds.len();
            
            if windleft && x>0 || !windleft && x<6 {
                let x_test = if windleft { x - 1 } else { x + 1 };
                let mut testok = true;
                'windtest: for r in 0..rockheight {
                    let y_test = y - r;
                    for c in 0..4 {
                        if rock[4*r + c] && ((x_test+c)>6 || (x_test+c)<7 && room[y_test][x_test+c]) {
                            testok = false;
                            break 'windtest;
                        }
                    }
                }
                if testok {
                    x = x_test;
                }
            }
            
            let mut testok = true;
            if y > 1 {
                'falltest: for r in 0..rockheight {
                    let y_test = y - r - 1;
                    for c in 0..4 {
                        if rock[4*r + c] && room[y_test][x + c] {
                            testok = false;
                            break 'falltest;
                        }
                    }
                }
            }
            if testok && y>0 {
                y -= 1;
            } else {
                break 'falloop;
            }
        }
        
        for r in 0..rockheight {
            for c in 0..4 {
                if rock[r*4 + c] {
                    room[y - r][x + c] = true;
                }
            }
        }

        let towerinc = max(toprow, y + 1) - toprow;
        toprow += towerinc;
        towerheight += towerinc;

        i += 1;

        if i >= 1000000000000 {
            break;
        } else if i == 2021 {
            result1 = towerheight;
        }
    }
    
    (result1, towerheight)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 17: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
