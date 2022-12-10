use std::fs;

fn solve(input: &str) -> i32 {
    let mut values: Vec<i32> = Vec::new();
    let mut x: i32 = 1;
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "addx" => {
                values.push(x);
                values.push(x);
                x += parts[1].parse::<i32>().unwrap();
            },
            _ => { values.push(x); }
        }
    }

    let mut screen = [[false; 40]; 6];
    
    let mut sum = 0;
    let mut i = 0;
    while i < values.len() {
        let row = i / 40;
        let col = i % 40;
        let pixel = (values[i]-1) <= col as i32 && col as i32 <= (values[i]+1);
        screen[row][col] = pixel;
        
        if (i+20)%40 == 0 {
            let v = values[i-1];
            sum += i as i32 * v;
        }
        i += 1;
    }

    for r in 0..6 {
        for c in 0..40 {
            print!("{}", if screen[r][c] { "#" } else { " " });
        }
        println!("");
    }
    
    sum
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 10: {}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
