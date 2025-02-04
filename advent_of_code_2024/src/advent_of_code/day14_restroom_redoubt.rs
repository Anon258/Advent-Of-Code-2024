use std::fs;
use std::path::Path;

use std::cmp::Ordering;

fn read_inputs(filename: &Path) -> Vec<((i32,i32), (i32,i32))>{
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");

    let robots = content.split("\n")
                                        .map(|line| parse_line(line))
                                        .collect();
    robots
}

fn parse_line(line: &str) -> ((i32,i32), (i32,i32)) {
    let (p,v) = line.split_once(" ").unwrap();
    let p = p[2..].split_once(",").unwrap();
    let v = v[2..].split_once(",").unwrap();

    ((p.0.parse().unwrap(), p.1.parse().unwrap()), (v.0.parse().unwrap(), v.1.parse().unwrap()))
}

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

fn check_pattern(grid: &Vec<Vec<i32>>, w: i32) -> bool {
    for i in 0..HEIGHT as usize {
        for start_j in 0..(WIDTH-w) as usize {
            let mut found = true;
            for j in 0..w as usize {
                if grid[start_j + j][i] == 0 {
                    found = false;
                    break;
                }
            }
            if found { return true }
        }
    }
    false
}

pub fn part1() {
    let robots = read_inputs(Path::new("input/day14.txt"));

    let mut grid_safety = [0,0,0,0];
    let time = 100;

    let middle = (WIDTH/2,HEIGHT/2);

    for (p,v) in robots {
        let new_pos = (((p.0 + time * v.0) % WIDTH + WIDTH) % WIDTH, (((p.1 + time * v.1) % HEIGHT) + HEIGHT) % HEIGHT);
        match (new_pos.0.cmp(&middle.0), new_pos.1.cmp(&middle.1)) {
            (Ordering::Less, Ordering::Less) => { grid_safety[0] += 1; },
            (Ordering::Less, Ordering::Greater) => { grid_safety[1] += 1; },
            (Ordering::Greater, Ordering::Less) => { grid_safety[2] += 1; },
            (Ordering::Greater, Ordering::Greater) => { grid_safety[3] += 1; },
            _ => continue,
        }
    }

    let ans = grid_safety.iter().fold(1, |acc, x| acc * x);
    println!("{}", ans);

}

pub fn part2() {
    let mut robots = read_inputs(Path::new("input/day14.txt"));

    let mut grid: Vec<Vec<i32>> = (0..WIDTH).map(|_| vec![0;  HEIGHT as usize]).collect();
    for robot in &robots {
        grid[robot.0.0 as usize][robot.0.1 as usize] += 1;
    }

    let mut time = 0;

    loop {
        robots.iter_mut().for_each(|(x,v)| {
            *x = (x.0 + v.0, x.1 + v.1);
            *x = ((x.0 % WIDTH + WIDTH) % WIDTH, (x.1 % HEIGHT + HEIGHT) % HEIGHT);
        });

        grid = (0..WIDTH).map(|_| vec![0;  HEIGHT as usize]).collect();
        for robot in &robots {
            grid[robot.0.0 as usize][robot.0.1 as usize] += 1;
        }

        time += 1;
        if check_pattern(&grid, 8) { break; }
    } 

    println!("{}", time);
}