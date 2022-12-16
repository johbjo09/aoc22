use std::fs;
use std::cmp::{min,max};
use regex::Regex;
use std::f64::consts::SQRT_2;

struct Sensor {
    x: i64,
    y: i64,
    x_b: i64,
    y_b: i64,
    distance: i64,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,    
    rx_min: i64,
    rx_max: i64,
    ry_min: i64,
    ry_max: i64,
}

fn rotate(x: f64, y: f64) -> (f64, f64) {
    ((x - y)/SQRT_2, (x + y)/SQRT_2)
}

fn rotateb(x: f64, y: f64) -> (f64, f64) {
    ((x + y)/SQRT_2, (y - x)/SQRT_2)
}

impl Sensor {
    fn new(x: i64, y: i64, x_b: i64, y_b: i64) -> Sensor {
        let d = (x_b - x).abs() + (y_b - y).abs();
        let (rx,ry) = rotate(x as f64, y as f64);
        let w = (d as f64)/SQRT_2;
        
        Sensor { x: x,
                 y: y,
                 x_b: x_b,
                 y_b: y_b,
                 distance: d,
                 
                 x_min: x - d,
                 x_max: x + d,
                 y_min: y - d,
                 y_max: y + d,
                 
                 rx_min: (rx - w).round() as i64,
                 rx_max: (rx + w).round() as i64,
                 ry_min: (ry - w).round() as i64,
                 ry_max: (ry + w).round() as i64,
        }
    }
    
    fn covers(&self, x: i64, y: i64) -> bool {
        self.distance >= ((x - self.x).abs() + (y - self.y).abs())
    }
}

fn solve(input: &str) -> (i64,i64) {
    let re = Regex::new(r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)").unwrap();
    let mut sensors = vec![];
    
    for line in input.lines() {
        if let Some(l) = re.captures(line) {
            let mut coords = [0_i64; 4];
            for i in 0..4 {
                coords[i] = l[i+1].parse::<i64>().unwrap();
            }
            sensors.push(Sensor::new(coords[0], coords[1], coords[2], coords[3]));
        }
    }
    
    let mut count = 0;
    {
        let y = 2000000;
        let mut x_min = std::i64::MAX;
        let mut x_max = std::i64::MIN;
        for s in &sensors {
            if s.y_min <= y && y <= s.y_max {
                x_min = min(x_min, s.x_min);
                x_max = max(x_max, s.x_max);
            }
        }

        for x in x_min..x_max {
            let mut covered = false;
            let mut blocked = false;
            for s in &sensors {
                if s.covers(x, y) {
                    covered = true;
                }
                if (s.x==x && s.y==y) || (s.x_b==x && s.y_b==y) {
                    blocked = true;
                }
            }
            if covered && !blocked {
                count += 1;
            }
        }
    }
    
    let xbound = 4000_000;
    let ybound = 4000_000;
    let mut intersections = vec![];
    
    for i in 0..sensors.len()-1 {
        let s1 = &sensors[i];
        for j in i+1..sensors.len() {
            let s2 = &sensors[j];
            intersections.push((s1.rx_min, s2.ry_min));
            intersections.push((s1.rx_min, s2.ry_max));
            intersections.push((s1.rx_max, s2.ry_min));
            intersections.push((s1.rx_max, s2.ry_max));
        }
    }
    
    let mut code = 0;
    
    for (rcx, rcy) in &intersections {
        let (xf,yf) = rotateb(*rcx as f64, *rcy as f64);
        let cx = xf.round() as i64;
        let cy = yf.round() as i64;
        for x in cx-1..cx+1 {
            for y in cy-1..cy+1 {
                if x>0 && y>0 && x<xbound && y<ybound {
                    let mut blocked = false;
                    let mut covered = false;
                    for s in &sensors {
                        if s.covers(x,y) {
                            covered = true;
                        }
                        if (s.x==x && s.y==y) || (s.x_b==x && s.y_b==y) {
                            blocked = true;
                        }
                    }
                    if !blocked && !covered {
                        code = x*4000000 + y;
                    }
                }
            }
        }
    }    
    (count, code)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 15: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
