use std::fs;
use regex::Regex;
use std::collections::HashMap;

enum Oper { Add, Subtract, Multiply, Divide }
use Oper::{*};

enum Node {
    Num(bool, i64),
    Op(bool, Oper, Box<Node>, Box<Node>),
}

impl Node {
    fn compute(&self) -> i64 {
        match self {
            Node::Num(_, n) => *n,
            Node::Op(_, op, l, r) => {
                let l = l.compute();
                let r = r.compute();
                match op { Add => l + r,
                           Subtract => l - r,
                           Multiply => l * r,
                           Divide => l / r }
            }
        }
    }
    
    fn ismark(&self) -> bool {
        match self {
            Node::Num(m, _) => *m,
            Node::Op(m, _, _, _) => *m,
        }
    }
    
    fn inv(&self, diff: i64) -> i64 {
        match self {
            Node::Num(true, _) => diff,
            Node::Op(true, op, l, r) =>  if l.ismark() {
                let r = r.compute();
                l.inv(match op { Add => diff - r,
                                 Subtract => diff + r,
                                 Multiply => diff / r,
                                 Divide => diff * r })
            } else {
                let l = l.compute();
                r.inv(match op { Add => diff - l,
                                 Subtract => l - diff,
                                 Multiply => diff / l,
                                 Divide => l / diff })
            },
            _ => self.compute()
        }
    }
}

fn make_node(lines: &HashMap<&str, &str>, name: &str, mark: &str) -> Node {
    let re = Regex::new(r"([a-z]+) ([+\-*/]) ([a-z]+)").unwrap();
    let line = lines.get(name).unwrap();
    if let Some(l) = re.captures(line) {
        let nl = make_node(lines, &l[1], mark);
        let nr = make_node(lines, &l[3], mark);        
        Node::Op(name.eq(mark) || nl.ismark() || nr.ismark(),
                 match &l[2] {"+" => Add, "-" => Subtract, "*" => Multiply, _ => Divide },
                 Box::new(nl),
                 Box::new(nr))
    } else {
        Node::Num(name.eq(mark),
                  line.parse::<i64>().unwrap())
    }
}

fn solve(input: &str) -> (i64,i64) {
    let mut lines = HashMap::new();    
    for line in input.lines() {
        let s: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        lines.insert(s[0], s[1]);
    }
    let root = make_node(&lines, "root", "humn");
    let answer1 = root.compute();
    let mut answer2 = 0;
    if let Node::Op(_, _, ln, rn) = root {
        answer2 = if rn.ismark() {
            rn.inv(ln.compute())
        } else {
            ln.inv(rn.compute())
        }
    }
    (answer1,answer2)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 21: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
