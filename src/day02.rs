use std::fs;

fn count_scores(contents: String) -> (u32, u32) {
    let mut scores = (0,0);
    
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            let opmove = parts[0];
            let mymove = parts[1];
            scores.0 += match (opmove,mymove) {
                ("A","X") => 1 + 3, // draw
                ("A","Y") => 2 + 6, // win
                ("A","Z") => 3,     // lose
                ("B","X") => 1,     // lose
                ("B","Y") => 2 + 3, // draw
                ("B","Z") => 3 + 6, // win
                ("C","X") => 1 + 6, // win
                ("C","Y") => 2,     // lose
                ("C","Z") => 3 + 3, // draw
                _ => 0
            };
            scores.1 += match (opmove,mymove) {
                ("A","X") => 3,     // lose
                ("A","Y") => 1 + 3, // draw
                ("A","Z") => 2 + 6, // win
                ("B","X") => 1,     // lose
                ("B","Y") => 2 + 3, // draw
                ("B","Z") => 3 + 6, // win
                ("C","X") => 2,     // lose
                ("C","Y") => 3 + 3, // draw
                ("C","Z") => 1 + 6, // win
                _ => 0
            };
        }
    }
    scores
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 02: {:?}", count_scores(contents)),
        Err(err) => println!("{}", err)
    }
}
