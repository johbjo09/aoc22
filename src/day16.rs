use std::fs;
use regex::Regex;
use std::cmp::max;
use std::collections::VecDeque;

fn search1(start: usize, timelimit: usize, flows: &Vec<usize>, openable: &Vec<usize>, mindists: &Vec<Vec<usize>>) -> usize
{
    let mut max_flow = 0;
    let allopen = openable.iter().fold(0, |a,i| a | 1 << i);
    let mut stack: Vec<(usize, usize, usize, usize)> = vec![];
    stack.push((start, 0, 0, 0));
        
    while stack.len() > 0 {
        let (i, time, valves, flow) = stack.pop().unwrap();
        max_flow = max(flow, max_flow);
        if valves != allopen {
            for &j in openable {
                if i != j {
                    let m = 1 << j;
                    if valves & m == 0 {
                        let newtime = time + &mindists[i][j];
                        if newtime < timelimit {
                            stack.push((j, newtime, valves | m, flow + flows[j] * (timelimit - newtime)));
                        }
                    }
                }
            }
        }
    }
    max_flow
}

fn search2(start: usize, timelimit: usize, flows: &Vec<usize>, openable: &Vec<usize>, mindists: &Vec<Vec<usize>>) -> usize
{
    let mut max_flow = 0;
    let allopen = openable.iter().fold(0, |a,i| a | 1 << i);
    let mut visits = vec![vec![(0,0); timelimit]; timelimit];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, start, 0, 0, 0));
    
    while queue.len() > 0 {
        let (i1, time1, i2, time2, valves, flow) = queue.pop_front().unwrap();
        max_flow = max(flow, max_flow);        
        if valves != allopen {
            for &j in openable {
                let m = 1 << j;
                if valves & m == 0 {
                    let newtime1 = time1 + &mindists[i1][j];
                    let newtime2 = time2 + &mindists[i2][j];
                    let newvalves = valves | m;
                    if newtime1 < timelimit  {
                        let (v1,f1) = visits[newtime1][time2];
                        let newflow1 = flow + flows[j] * (timelimit - newtime1);
                        if v1 & m == 0 || f1 < newflow1 {
                            visits[newtime1][time2] = (v1 | newvalves, max(f1, newflow1));
                            queue.push_back((j, newtime1, i2, time2, newvalves, newflow1));
                        }
                    }
                    if newtime2 < timelimit {
                        let (v2,f2) = visits[time1][newtime2];
                        let newflow2 = flow + flows[j] * (timelimit - newtime2);
                        if v2 & m == 0 || f2 < newflow2 {
                            visits[time1][newtime2] = (v2 | newvalves, max(f2, newflow2));
                            queue.push_back((i1, time1, j, newtime2, newvalves, newflow2));
                        }
                    }
                }
            }
        }
    }    
    max_flow
}

fn find_minpaths(graph: &Vec<Vec<usize>>, from: usize, to: usize) -> Vec<usize> {
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    queue.push_back(vec![from]);    
    while queue.len() > 0 {
        let path = queue.pop_front().unwrap();
        let &i = path.last().unwrap();            
        for &j in &graph[i] {
            if i != j && !path.contains(&j) {
                if j == to {
                    let mut minpath = Vec::from(path.as_slice());
                    minpath.push(j);
                    return minpath;
                } else {
                    queue.push_back(Vec::from(path.as_slice()));
                    let newpath = queue.back_mut().unwrap();
                    newpath.push(j);
                }
            }
        }
    }    
    vec![]
}

struct Line {
    label: String,
    flow: usize,
    tunnels: Vec<String>,
}

fn solve(input: &str) -> (usize,usize) {
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)").unwrap();    

    let data: Vec<Line> = input.lines()
        .map(|line| {
            let l = re.captures(line).unwrap();
            let flow = l[2].parse::<usize>().unwrap();
            Line { label: String::from(&l[1]),
                   flow: flow,
                   tunnels: l[3].split(",").map(|s| s.trim().to_string()).collect() } })
        .collect();
    
    let len = data.len();
    
    let graph = (0..len).map(|i|
                             data[i].tunnels
                             .iter()
                             .map(|tunnel| (0..len).filter(|&j| data[j].label.eq(tunnel)).nth(0).unwrap())
                             .collect()).collect();
    let flows = data.iter().map(|v| v.flow).collect();
    let openable = (0..len).filter(|&i| data[i].flow > 0).collect();

    let mut mindists = vec![vec![0; len]; len];
    for i in 0..len {
        for j in i+1..len {
            let path = find_minpaths(&graph, i, j);
            mindists[i][j] = path.len();
            mindists[j][i] = path.len();
        }
    }
    
    let start = (0..len).filter(|&i| data[i].label.eq("AA")).nth(0).unwrap();

    let maxflow1 = search1(start, 30, &flows, &openable, &mindists);
    let maxflow2 = search2(start, 26, &flows, &openable, &mindists);
    
    (maxflow1, maxflow2)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 16: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
