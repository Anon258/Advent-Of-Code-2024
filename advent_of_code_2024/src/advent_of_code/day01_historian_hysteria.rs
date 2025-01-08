use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// ============================================================= FILE I/O ====================================================================
fn read_inputs(filename: &Path) -> (Vec::<i32>, Vec::<i32>) {
    let Ok(lines) = read_lines(filename) else {
        std::process::exit(1);
    };

    let mut l1:Vec::<i32> = Vec::new();
    let mut l2:Vec::<i32> = Vec::new();  

    for line in lines.flatten() {
        let nums: Vec::<i32> = line.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();
        
        l1.push(nums[0]);
        l2.push(nums[1]);
    }
    (l1, l2)
}

fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).expect("Unable to read file");
    Ok(io::BufReader::new(file).lines())
}

// ===========================================================================================================================================

// ========================================================== Helper Functions ===============================================================
fn merge_sort(vec: &Vec::<i32>) -> Vec::<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left_array = merge_sort(&vec[0..size].to_vec());
        let right_array = merge_sort(&vec[size..].to_vec());
        merge(&left_array, &right_array)
    }
}

fn merge(left_arr: &Vec::<i32>, right_arr: &Vec::<i32>) -> Vec::<i32> {
    let mut i = 0; 
    let mut j = 0;
    let mut merged_arr: Vec::<i32> = Vec::new();

    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] < right_arr[j] {
            merged_arr.push(left_arr[i]);
            i = i + 1;
        }
        else {
            merged_arr.push(right_arr[j]);
            j = j + 1;
        }
    }

    if i < left_arr.len() {
        while i < left_arr.len() {
            merged_arr.push(left_arr[i]);
            i = i + 1;
        }
    }

    if j < right_arr.len() {
        while j < right_arr.len() {
            merged_arr.push(right_arr[j]);
            j = j + 1;
        }
    }

    merged_arr
}

// ===========================================================================================================================================

pub fn part1() -> i32 {
    let (mut l1, mut l2) = read_inputs(Path::new("input/day01"));
    l1 = merge_sort(&l1);
    l2 = merge_sort(&l2);

    assert!(l1.len() == l2.len());

    let mut diff = 0;
    for i in 0..l1.len() {
        diff = diff + (l1[i] - l2[i]).abs();
    }

    // println!("Total Difference: {}", diff);
    diff
}

pub fn part2() -> i32 {
    let (l1, l2) = read_inputs(Path::new("input/day01"));

    let freq_l2 = l2.iter()
                    .fold(HashMap::new(), |mut map: HashMap<&i32, i32>, val| {
                        *map.entry(val).or_default() += 1;
                        map
                    });
    
    let mut sim = 0;
    for i in 0..l1.len() {
        match freq_l2.get(&l1[i]) {
            Some(&number) => sim += l1[i]*number,
            _ => (),
        }
    }
    sim
}