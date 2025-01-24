use std::collections::HashMap;
use std::fs;
use std::path::Path;


fn read_inputs(filename: &Path) -> HashMap<usize, usize> {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");

    content.split(' ')
            .map(|s| (s.parse().unwrap(),1))
            .collect()
    
}

fn make_stones(map: &mut HashMap<usize, usize>, num_iter: usize) -> usize {
    
    for _i in 0..num_iter {
        map.clone()
            .iter()
            .filter(|(_,&val)| val != 0 )
            .for_each(|(&key, &val)| {
                if key == 0 {
                    *map.get_mut(&key).unwrap() -= val;
                    let res = map.entry(1).or_insert(0);
                    *res += val;
                } else {
                    let stone_str = key.to_string();
                    if stone_str.len() % 2 == 0 {
                        let left_num = stone_str[0..stone_str.len()/2].parse().unwrap();
                        let right_num = stone_str[stone_str.len()/2..].parse().unwrap();

                        *map.get_mut(&key).unwrap() -= val;

                        let res = map.entry(left_num).or_insert(0);
                        *res += val;

                        let res = map.entry(right_num).or_insert(0);
                        *res += val;
                    } else {
                        *map.get_mut(&key).unwrap() -= val;
                        let res = map.entry(2024*key).or_insert(0);
                        *res += val;
                    }
                }
            });
    }

    map.values().sum()
}

pub fn part1() {
    let mut map = read_inputs(Path::new("input/day11.txt"));
    let ans = make_stones(&mut map, 25);

    println!("{}", ans);
}

pub fn part2() {
    let mut map = read_inputs(Path::new("input/day11.txt"));
    let ans = make_stones(&mut map, 75);

    println!("{}", ans);
}