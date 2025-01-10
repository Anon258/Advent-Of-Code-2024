use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_inputs(filename: &Path) -> Vec<(i64, Vec<i64>)> {
    let file = File::open(filename).expect("Unable to read file");
    let mut reader = io::BufReader::new(file);
    let mut _l = String::new();

    let mut queries: Vec<(i64, Vec<i64>)> = Vec::new();

    loop {
        let Ok(len) = reader.read_line(&mut _l) else { 
            break; };
        if len == 0 { break; }
        let eqs: Vec<&str> = _l[0.._l.len()-1].split(":").collect();

        let target = eqs[0].parse::<i64>().unwrap();
        let nums: Vec<i64> = eqs[1][1..].split(' ').map(|x| x.parse().unwrap()).collect();
        queries.push((target, nums));
        
        _l.clear();
    }

    queries
}

fn check_possible_basic(target: i64, nums: &Vec<i64>) -> i64 {
    let mut stack: Vec<(i32,i64)> = Vec::new();
    stack.push((0,nums[0]));

    while stack.len() > 0 {
        let state = if let Some(state) = stack.pop() { state } else { break; };

        if state.0 as usize == nums.len() - 1 {
            if state.1 == target { return target } else { continue; }
        }

        let new_val = state.1 + nums[(state.0 + 1) as usize];
        if new_val  <= target {
            stack.push((state.0 + 1, new_val));
        }

        let new_val = state.1 * nums[(state.0 + 1) as usize];
        if new_val  <= target {
            stack.push((state.0 + 1, new_val));
        }
    }

    0
}

fn check_possible_adv(target: i64, nums: &Vec<i64>) -> i64 {
    let mut stack: Vec<(i32,i64)> = Vec::new();
    stack.push((0,nums[0]));

    while stack.len() > 0 {
        let state = if let Some(state) = stack.pop() { state } else { break; };

        if state.0 as usize == nums.len() - 1 {
            if state.1 == target { return target } else { continue; }
        }

        let new_val = state.1 + nums[(state.0 + 1) as usize];
        if new_val  <= target {
            stack.push((state.0 + 1, new_val));
        }

        let new_val = state.1 * nums[(state.0 + 1) as usize];
        if new_val  <= target {
            stack.push((state.0 + 1, new_val));
        }

        let new_val = (state.1 * i64::pow(10, nums[(state.0 + 1) as usize].to_string().len() as u32)) + nums[(state.0 + 1) as usize];
        if new_val  <= target {
            stack.push((state.0 + 1, new_val));
        }
    }

    0
}

pub fn part1() {
    let queries = read_inputs(Path::new("input/day07.txt"));
    let mut ans = 0;
    for (target, nums) in queries {
        ans += check_possible_basic(target, &nums);
    }

    println!("{}",ans);
}

pub fn part2() {
    let queries = read_inputs(Path::new("input/day07.txt"));
    let mut ans = 0;
    for (target, nums) in queries {
        let res = check_possible_adv(target, &nums);
        // println!("{}",res);
        ans += res;
    }

    println!("{}",ans);
}