use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

// ============================================================= FILE I/O ====================================================================
fn read_inputs(filename: &Path) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let file = File::open(filename).expect("Unable to read file");
    let mut reader = io::BufReader::new(file);
    let mut _l = String::new();
    
    let mut map : HashMap<i32, HashSet<i32>> = HashMap::new();
    loop {
        let Ok(len) = reader.read_line(&mut _l) else { 
            break; };
        if len == 1 { break; }
        let nums: Vec<i32> = _l[0.._l.len()-1].split("|").map(|l| l.parse().unwrap()).collect();
        let list_set = map.entry(nums[0]).or_insert(HashSet::new());
        list_set.insert(nums[1]);
        
        _l.clear();
    }

    _l.clear();

    let mut queries: Vec<Vec<i32>> = Vec::new();

    loop {
        let Ok(len) = reader.read_line(&mut _l) else { 
            break; };
        if len == 0 { break; }
        queries.push(_l[0.._l.len()-1].split(',').map(|l| l.parse().unwrap()).collect());
        _l.clear();
    }

    _l.clear();

    return (map, queries);

}

// ===========================================================================================================================================

pub fn part1() -> i32 {
    let (map, queries) = read_inputs(Path::new("input/day05.txt"));

    let mut ans = 0; 
    for query in queries {
        let mut valid = true;

        for i in (0..query.len()-1).rev() {
            let _hashset = map.get(&query[i+1]);
            if _hashset.is_some_and(|_hashset| {_hashset.contains(&query[i])}) {
                valid = false;
                break;
            }
        }

        if valid {
            ans += query[query.len()/2];
        }
    }

    ans
} 

pub fn part2() -> i32 {
    let (map, queries) = read_inputs(Path::new("input/day05.txt"));

    let mut ans = 0; 
    for mut query in queries {
        let mut valid = true;

        for i in (0..query.len()-1).rev() {
            let _hashset = map.get(&query[i+1]);
            if _hashset.is_some_and(|_hashset| {_hashset.contains(&query[i])}) {
                valid = false;
                break;
            }
        }

        if valid {
            continue;
        }

        query.sort_by(|a,b| {
            if map.get(a).is_some_and(|x| x.contains(b)) { std::cmp::Ordering::Less }
            else if map.get(b).is_some_and(|x| x.contains(a)) { std::cmp::Ordering::Greater }
            else { std::cmp::Ordering::Equal }
        });

        ans += query[query.len()/2];
    }

    ans
}