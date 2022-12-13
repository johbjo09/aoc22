use std::fs;
use std::cmp::{Ordering, min};

enum Packet {
    Unit(u8),
    List(Vec<Packet>),
}

fn cmp_lists(s: &Vec<Packet>, o: &Vec<Packet>) -> Ordering {
    let n = min(s.len(), o.len());
    let mut i = 0;
    let mut ord = Ordering::Equal;
    while i<n && ord == Ordering::Equal {
        ord = s[i].cmp(&o[i]);
        i += 1;
    }
    if ord == Ordering::Equal && s.len() < o.len() {
        Ordering::Less
    } else if ord == Ordering::Equal && s.len() > o.len() {
        Ordering::Greater        
    } else {
        ord
    }
}

impl Packet {    
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self,other) {
            (Packet::Unit(s), Packet::Unit(o)) => s.cmp(&o),
            (Packet::List(s), Packet::List(o)) => cmp_lists(&s, &o),
            (Packet::List(s), Packet::Unit(o)) => cmp_lists(&s, &vec![Packet::Unit(*o)]),
            (Packet::Unit(s), Packet::List(o)) => cmp_lists(&vec![Packet::Unit(*s)], &o),
        }
    }
    
    fn show(&self) -> String {
        match self {
            Packet::Unit(n) => format!("{}", n),
            Packet::List(l) => {
                let inner: Vec<String> = l.iter().map(|n| n.show()).collect();
                format!("[{}]", inner.join(","))
            },
        }
    }
}

struct Parser {
    s: String,
    p: usize,
}

impl Parser {
    fn new(s_: &str) -> Parser {
        Parser { s: String::from(s_),
                 p: 0 }
    }
    
    fn parse(&mut self) -> Packet {
        let mut list = vec![];
        self.p += 1;
        let mut q = self.p;
        while self.p < self.s.len() {
            let c = self.s.chars().nth(self.p).unwrap();
            if c == '[' {
                list.push(self.parse());
                q = self.p+1;
            } else if c==',' || c==']' {
                if q < self.p {
                    list.push(Packet::Unit(self.s[q..self.p].parse().unwrap()));
                }
                if c==']' {
                    break;
                }
                q = self.p+1;
            }
            self.p += 1;
        }
        return Packet::List(list);
    }
}

fn parse(s: &str) -> Packet {
    let mut parser = Parser::new(s);
    parser.parse()
}

fn solve1(input: &str) -> u32 {
    let mut i = 0;
    let mut count = 0;
    let mut lines = input.lines();
    loop {
        if let Some(line1) = lines.next() {
            if line1.len() == 0 {
                continue;
            }            
            let packet1 = parse(line1);
            if let Some(line2) = lines.next() {
                let packet2 = parse(line2);
                i += 1;
                if packet1.cmp(&packet2) == Ordering::Less {
                    count += i;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }    
    count
}

fn solve2(input: &str) -> usize {
    let mut list = vec![];
    
    for line in input.lines() {
        if line.len() > 0 {
            let packet = parse(line);
            list.push(packet);
        }
    }
    list.push(parse("[[2]]"));
    list.push(parse("[[6]]"));

    list.sort_by(|a, b| a.cmp(b));

    let mut key = 1;
    for i in 0..list.len() {
        let line = list[i].show();
        if line.eq("[[2]]") || line.eq("[[6]]") {
            key *= i+1;
        }
    }
    key
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 13: {} {}", solve1(&contents), solve2(&contents)),
        Err(err) => println!("{}", err),
    }
}
