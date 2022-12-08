use std::fs;

struct Dir<'a> {
    name: &'a str,
    size: usize,
    parent: usize,
}

struct Tree<'a> {
    dirs: Vec<Dir<'a>>,
}

impl<'a> Tree<'a> {
    fn add_subdir(&mut self, parent: usize, name: &'a str) -> usize {
        let i = self.dirs.len();
        self.dirs.push(Dir { name: name,
                             size: 0,
                             parent: parent, });
        i
    }

    fn add_size(&mut self, dir: usize, size: usize) {
        self.dirs[dir].size += size;
        if dir != 0 {
            self.add_size(self.dirs[dir].parent, size);
        }
    }

    fn find_subdir(&self, parent: usize, name: &'a str) -> Option<usize> {
        (parent+1..self.dirs.len())
            .find(|&i| self.dirs[i].parent==parent && self.dirs[i].name == name)
    }
}

fn solve(input: &str) -> (usize,usize) {
    let mut tree = Tree { dirs: Vec::new() };
    let mut listing = false;
    let mut cur = 0;
    
    for line in input.lines() {
        if line.contains("$") {
            listing = false;
            if line.contains("$ cd") {
                let parts: Vec<&str> = line.split(' ').collect();
                let dirname = parts[2];
                if dirname.eq("/") {
                    cur = 0;
                } else if dirname.eq("..") {
                    cur = tree.dirs[cur].parent;
                } else {
                    if let Some(dir) = tree.find_subdir(cur, dirname) {
                        cur = dir;
                    }
                }
            } else if line.eq("$ ls") {
                listing = true;
            }
        } else if listing {            
            let parts: Vec<&str> = line.split(' ').collect();
            if line.contains("dir ") && parts.len() == 2 {
                let dirname = parts[1];
                tree.add_subdir(cur, dirname);
            } else {
                let sz = parts[0].parse::<usize>().unwrap();
                tree.add_size(cur, sz);
            }
        }
    }

    let needspace = tree.dirs[0].size + 30000000 - 70000000;
    let mut minsize = std::usize::MAX;
    let mut total = 0;
    for i in 1..tree.dirs.len() {
        let sz = tree.dirs[i].size;
        if sz < 100000 {
            total += sz;
        }
        if sz > needspace && sz < minsize {
            minsize = sz;
        }        
    }
    
    (total, minsize)
}

pub fn solution(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Day 07: {:?}", solve(&contents)),
        Err(err) => println!("{}", err),
    }
}
