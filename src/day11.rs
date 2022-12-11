use std::fs;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    op: char,
    operand: u64,
    divisor: u64,
    targets: [usize; 2],
    inspections: u64,
}

fn read_monkeys(input: &str, monkeys: &mut Vec<Monkey>) {
    let mut lines = input.lines();
    let mut done = false;
    
    while !done {
        if let Some(line) = lines.next() {
            if line.contains("Monkey") {
                monkeys.push(Monkey { items: VecDeque::new(), op: ' ', operand: 0, divisor: 1, targets: [0, 0], inspections: 0 });
                let monkey = monkeys.last_mut().unwrap();
                let line = lines.next().unwrap();
                if line.contains("Starting") {
                    let parts: Vec<&str> = line[18..line.len()].split(',').collect();
                    for p in parts {
                        monkey.items.push_back(p.trim().parse::<u64>().unwrap());
                    }
                }
                let line = lines.next().unwrap();
                if line.contains("Operation") {
                    monkey.op = line.chars().nth(23).unwrap();
                    if let Ok(opr) = line[25..line.len()].trim().parse::<u64>() {
                        monkey.operand = opr;
                    }
                }
                let line = lines.next().unwrap();
                if line.contains("Test: divisible") {
                    monkey.divisor = line[21..line.len()].parse::<u64>().unwrap();
                    let line = lines.next().unwrap();
                    if line.contains("If true") {
                        monkey.targets[1] = line[29..line.len()].parse::<usize>().unwrap();
                    }
                    let line = lines.next().unwrap();
                    if line.contains("If false") {
                        monkey.targets[0] = line[30..line.len()].parse::<usize>().unwrap();                        
                    }                    
                }
            }
        } else {
            done = true;
        }
    }

}

fn count(monkeys: &Vec<Monkey>) -> u64 {
    let mut inspections = [0_u64; 2];
    let mut low_i = 0;
    for im in 0..monkeys.len() {
        let monkey = &monkeys[im];
        if inspections[low_i] < monkey.inspections {
            inspections[low_i] = monkey.inspections;
            for i in 0..2 {
                if inspections[low_i] > inspections[i] {
                    low_i = i;
                }                   
            }
        }
    }
    inspections.iter().product()
}

fn solve1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    read_monkeys(&input, &mut monkeys);

    for _ in 0..20 {
        for im in 0..monkeys.len() {
            let monkey = &mut monkeys[im];
            let mut throws = vec![];
            while monkey.items.len() > 0 {
                let item = monkey.items.pop_front().unwrap();
                let operand = if monkey.operand == 0 { item } else { monkey.operand };
                let item = match monkey.op {
                    '*' => item * operand,
                    '+' => item + operand,
                    _ => item,
                };
                let item = (item as f32 / 3.0).floor() as u64;
                let target_idx = (item % monkey.divisor == 0) as usize;
                monkey.inspections += 1;
                let target = monkey.targets[target_idx];
                throws.push((target,item));
            }
            for (target,item) in throws {
                monkeys[target].items.push_back(item);
            }
        }
    }
    count(&monkeys)
}

fn solve2(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    read_monkeys(&input, &mut monkeys);
    
    let mut modgroup = 1;
    for im in 0..monkeys.len() {
        modgroup *= monkeys[im].divisor;
    }
    
    for _ in 0..10000 {
        for im in 0..monkeys.len() {
            let monkey = &mut monkeys[im];
            let mut throws = vec![];
            while monkey.items.len() > 0 {
                monkey.inspections += 1;
                let item = monkey.items.pop_front().unwrap();
                let operand = if monkey.operand == 0 { item } else { monkey.operand };
                let item = match monkey.op {
                    '*' => item * operand,
                    '+' => item + operand,
                    _ => item,
                };
                let item = item % modgroup;
                let target_idx = (item % monkey.divisor == 0) as usize;
                let target = monkey.targets[target_idx];
                throws.push((target,item));
            }
            for (target,item) in throws {
                monkeys[target].items.push_back(item);
            }
        }
    }
    count(&monkeys)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 11: {} {}", solve1(&contents), solve2(&contents)),
        Err(err) => println!("{}", err),
    }
}
