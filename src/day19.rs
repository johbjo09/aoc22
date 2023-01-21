
use std::fs;
use std::cmp::max;
use regex::Regex;

fn search(timelimit: usize,
          c_ore: usize,
          c_clay: usize,
          c_obs_ore: usize,
          c_obs_clay: usize,
          c_geo_ore: usize,
          c_geo_obs: usize) -> usize
{
    let max_c_ore = max(c_geo_ore, max(c_clay, c_obs_ore));
    let mut stack = vec![];
    let mut max_value = vec![0; timelimit as usize];
    stack.push((timelimit, 1, 0, 0, 0, 0, 0, 0, 0, 0));
    while stack.len() > 0 {
        let (t_, ore_bots, clay_bots, obs_bots, ore, clay, obs, geo, greedy, p) = stack.pop().unwrap();
        let t = t_ - 1;
        if max_value[t] <= greedy {
            let next_ore = ore + ore_bots;
            let next_clay = clay + clay_bots;
            let next_obs = obs + obs_bots;
            if ore >= c_geo_ore && obs >= c_geo_obs {
                let next_geo = geo + t;
                if max_value[t_] < next_geo { 
                    max_value[t_] = next_geo;
                }
                if t > 1 {
                    let next_ore = next_ore - c_geo_ore;
                    let next_obs = next_obs - c_geo_obs;
                    stack.push((t, ore_bots, clay_bots, obs_bots, next_ore, next_clay, next_obs, next_geo, greedy, 0));
                }
            } else if t > 1 { 
                if p < 4 {
                    stack.push((t, ore_bots, clay_bots, obs_bots, next_ore, next_clay, next_obs, geo, greedy, p + 1));
                }
                if ore >= c_ore && ore_bots < max_c_ore && t > (max_c_ore + c_ore + 1)   {
                    stack.push((t, ore_bots + 1, clay_bots, obs_bots, next_ore - c_ore, next_clay, next_obs, geo, greedy, 0));
                }
                if ore >= c_clay && (clay_bots < c_obs_clay) && t > 3 {
                    stack.push((t, ore_bots, clay_bots + 1, obs_bots, next_ore - c_clay, next_clay, next_obs, geo, greedy, 0));
                }
                if ore >= c_obs_ore && clay >= c_obs_clay && (obs_bots < c_geo_obs) && t > 2 {
                    let next_ore = next_ore - c_obs_ore;
                    let next_clay = next_clay - c_obs_clay;
                    let mut greedy = geo;
                    let mut g_ore = next_ore;
                    let mut g_obs = next_obs;
                    for gt in (1..t).rev() {
                        g_ore += ore_bots;
                        g_obs += obs_bots + 1;
                        if g_ore >= c_geo_ore && g_obs >= c_geo_obs {
                            greedy += gt;
                            g_ore -= c_geo_ore;
                            g_obs -= c_geo_obs;
                        }
                    }
                    stack.push((t, ore_bots, clay_bots, obs_bots + 1, next_ore, next_clay, next_obs, geo, greedy, 0));
                }
            }
        }
    }
    *max_value.iter().max().unwrap()
}

fn solve(input: &str) -> (usize,usize) {    
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let mut count = 0;
    let mut score1 = 0;
    let mut score2 = 1;    
    for line in input.lines() {
        if let Some(l) = re.captures(line.trim()) {
            let values: Vec<usize> = (1..l.len()).map(|i| l[i].trim().parse().unwrap()).collect();
            let max_geode1 = search(24, values[1], values[2], values[3], values[4], values[5], values[6]);
            score1 += max_geode1 * values[0];
            if count < 3 {
                let max_geode2 = search(32, values[1], values[2], values[3], values[4], values[5], values[6]);
                score2 *= max_geode2;
                count += 1;
            }
        }         
    }
    (score1 as usize, score2 as usize)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 19: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
