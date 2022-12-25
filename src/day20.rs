use std::fs;
use std::ptr;

struct List {
    n: i64,
    next: *mut List,
    prev: *mut List,
}

fn mix(ns: &mut Vec<List>, rounds: usize) -> i64 {
    let len = ns.len();
    let mut answer = 0;
    unsafe {
        for _ in 0..rounds {
            for i in 0..len {
                let n = ns[i].n % (len-1) as i64;
                let mut item = &mut ns[i];
                (*(*item).prev).next = (*item).next;
                (*(*item).next).prev = (*item).prev;
                if n>0 {
                    let mut tmp = (*item).next;
                    for _ in 0..n { tmp = (*tmp).next; }
                    item.next = tmp;
                    item.prev = (*tmp).prev;
                } else {
                    let mut tmp = (*item).prev;
                    for _ in 0..n.abs() { tmp = (*tmp).prev; }
                    item.next = (*tmp).next;
                    item.prev = tmp;
                }
                (*(*item).next).prev = item;
                (*(*item).prev).next = item;
            }
        }
        let count = 1000 % len;
        let mut item = ns[0].next;
        while (*item).n != 0 { item = (*item).next; }
        for _ in 0..count { item = (*item).next; }
        answer += (*item).n;
        for _ in 0..count { item = (*item).next; }
        answer += (*item).n;
        for _ in 0..count { item = (*item).next; }
        answer += (*item).n;
    }
    answer
}

fn solve(input: &str) -> (i64,i64) {
    let is: Vec<i64> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    let len = is.len();
    let mut ns = vec![];
    for &i in &is {
        ns.push(List { n: i, prev: ptr::null_mut(), next: ptr::null_mut() });
    }
    
    for i in 0..len {
        if i > 0 { ns[i-1].next = &mut ns[i]; }
        if i < len-1 { ns[i+1].prev = &mut ns[i]; }
    }
    ns[0].prev = &mut ns[len-1];
    ns[len-1].next = &mut ns[0];
    let answer1 = mix(&mut ns, 1);

    for i in 0..len {
        ns[i].n *= 811589153;
        if i > 0 { ns[i-1].next = &mut ns[i]; }
        if i < len-1 { ns[i+1].prev = &mut ns[i]; }
    }
    ns[0].prev = &mut ns[len-1];
    ns[len-1].next = &mut ns[0];
    let answer2 = mix(&mut ns, 10);

    (answer1,answer2)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 20: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
