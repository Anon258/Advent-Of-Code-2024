use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use itertools::Itertools;

fn read_inputs(filename: &Path) -> (HashMap<char, Vec<(i32,i32)>>, (i32,i32)) {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    
    let mut height = 0;
    let mut width = 0;

    let res: HashMap<char, Vec<(i32,i32)>> = content.split('\n').enumerate()
                                                    .fold(HashMap::new(), |mut map, (idx,str)| {
                                                        height += 1;
                                                        width = str.len() as i32;
                                                        str.chars().enumerate()
                                                            .filter(|(_r,ch)| *ch != '.')
                                                            .for_each(|(r,ch)| {
                                                                map.entry(ch)
                                                                    .and_modify(|v| v.push((idx as i32, r as i32)))
                                                                    .or_insert(vec![(idx as i32, r as i32)]);
                                                            });
                                                        map
                                                    });

    // println!("{:?}",(height, width));
    (res, (height, width))
}

pub fn part1() {
    let (inputs, (height,width)) = read_inputs(Path::new("input/day08.txt"));

    let res: HashSet<(i32,i32)> = inputs.into_iter().flat_map(|(_key, val)| {
                        val.iter().permutations(2)
                            .flat_map(|points| {
                                let new_x = 2 * points[1].0 - points[0].0;
                                let new_y = 2 * points[1].1 - points[0].1;
                                if new_x < width && new_x >= 0 && new_y < height && new_y >= 0 {
                                    Some((new_x, new_y))
                                } else {
                                    None
                                }
                            } ).collect::<HashSet<(i32,i32)>>()
                    }).collect();
    println!("{}",res.len());
}

pub fn part2() {
    let (inputs, (height,width)) = read_inputs(Path::new("input/day08.txt"));

    let res: HashSet<(i32,i32)> = inputs.into_iter().flat_map(|(_key, val)| {
                        val.iter().permutations(2)
                            .flat_map(|points| {
                                let mut new_x = points[1].0;
                                let mut new_y = points[1].1;
                                let mut antinodes: Vec<(i32,i32)> = Vec::new();
                                loop {
                                    if new_x < width && new_x >= 0 && new_y < height && new_y >= 0 {
                                        antinodes.push((new_x, new_y));
                                    } else { break; }
                                    new_x = new_x + (points[1].0 - points[0].0);
                                    new_y = new_y + (points[1].1 - points[0].1);
                                }
                                antinodes
                            } ).collect::<HashSet<(i32,i32)>>()
                    }).collect();
    println!("{}",res.len());
}