use std::fs;

fn count_calories(contents: String, sz: usize) -> (u32, u32) {
    let mut max: u32 = 0;
    let mut cals: u32 = 0;
    let mut cals_star: Vec<u32> = Vec::with_capacity(sz);

    for _ in 0..sz {
        cals_star.push(0);
    }
    
    for line in contents.lines() {
        if line.len() == 0 {
            if max < cals {
                max = cals;
            }
            for i in 0..sz {
                if cals_star[i] < cals {
                    cals_star[i] = cals;
                    break;
                }
            }
            cals = 0;
        }
        if let Ok(line_calories) = line.parse::<u32>() {
            cals += line_calories;
        }
    }

    (max, cals_star.iter().sum())
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 01: {:?}", count_calories(contents, 3)),
        Err(err) => println!("{}", err)
    }
}
