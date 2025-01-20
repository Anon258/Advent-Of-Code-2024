use std::cmp::min;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::u64;

fn read_inputs(filename: &Path) -> Vec<u8> {
    let file = File::open(filename).expect("Unable to read file");
    let mut reader = io::BufReader::new(file);

    let mut l = String::new();
    let Ok(_len) = reader.read_line(&mut l) else { 
        exit(1); };
    
    let nums : Vec<u8> = l.into_bytes().iter().map(|x| x - ('0' as u8)).collect();

    nums

}

pub fn part1() {
    let nums = read_inputs(Path::new("input/day09.txt"));

    let mut files: Vec<u64> = nums.iter().enumerate().filter(|(idx, _num)| idx % 2 == 0).map(|(_idx,num)| *num as u64).collect();
    let mut gaps: Vec<u64> = nums.iter().enumerate().filter(|(idx, _num)| idx % 2 == 1).map(|(_idx,num)| *num as u64).collect();

    let mut left_file_idx: usize  = 0;
    let mut right_file_idx: usize = files.len() - 1;
    let mut left_gap_idx: usize = 0;
    let mut base: u64 = 0;
    let mut hashsum: u64= 0;
    loop {

        let len = files[left_file_idx];
        hashsum += (left_file_idx as u64) * (len * (2 * base + len - 1)) / 2;
        base += len;
        // println!("Current Hash: {}", hashsum);
        left_file_idx += 1;

        if left_file_idx > right_file_idx { break; }

        loop
        {
            let space = min(gaps[left_gap_idx], files[right_file_idx]);

            gaps[left_gap_idx] -= space;
            files[right_file_idx] -= space;

            hashsum += (right_file_idx as u64) * (space * (2 * base + space - 1)) / 2;
            base += space;


            if files[right_file_idx] == 0 { right_file_idx -= 1;}

            if left_file_idx > right_file_idx { break; }

            if gaps[left_gap_idx] == 0 { left_gap_idx += 1; break; }

        }
    }

    println!("{}",hashsum);
}

pub fn part2()
{
    let nums = read_inputs(Path::new("input/day09.txt"));
    let mut size_heaps : Vec<BinaryHeap<Reverse<u64>>>= (0..10).map(|_x| BinaryHeap::<Reverse<u64>>::new()).collect();

    let mut start_idx = 0;

    for (idx, &size) in nums.iter().enumerate() {
        if idx % 2 == 1 {
            size_heaps[size as usize].push(Reverse(start_idx));
        }

        start_idx += size as u64;
    }

    for _i in 0..10 {
        size_heaps[_i].push(Reverse(start_idx));
    }

    let mut hashsum: u64 = 0;

    for (idx, &size) in nums.iter().enumerate().rev() {
        start_idx -= size as u64;
        
        if idx % 2 == 1 { continue; }

        let mut earliest_idx = start_idx;
        let mut heap_index = usize::MAX;

        for (i, heap) in size_heaps.iter().enumerate().skip(size as usize) {
            let Reverse(first_block) = heap.peek().unwrap();
            if *first_block < earliest_idx {
                earliest_idx = *first_block;
                heap_index = i ;
            }
        }

        let id = (idx / 2) as u64;
        let len = size as u64;
        hashsum += id * (len * (2 * earliest_idx + len - 1)) / 2;

        if heap_index != usize::MAX {
            size_heaps[heap_index].pop();
            size_heaps[heap_index - (size as usize)].push(Reverse(earliest_idx + len));
        }
    }

    println!("{}",hashsum);
}