use std::fs;
use std::path::Path;

use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

// ========================================================== INPUT STRUCT ===================================================================
#[derive(Debug)]
struct GridContext {
    grid: Vec<Vec<char>>,
    obstacles_row: Vec<Vec<i32>>,
    obstacles_col: Vec<Vec<i32>>,
    start_pos: (i32, i32),
}
// ===========================================================================================================================================

// ============================================================= FILE I/O ====================================================================
fn read_inputs(filename: &Path) -> GridContext {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    let grid: Vec<Vec<char>> = content.split("\n")
                                        .map(|s| {s.chars().collect()})
                                        .collect();

    let mut obstacles_row : Vec<Vec<i32>> = (0..grid.len()).map(|_| Vec::new()).collect();
    let mut obstacles_col : Vec<Vec<i32>> = (0..grid[0].len()).map(|_| Vec::new()).collect();
    let mut start_pos: (i32, i32) = (0,0);

    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                obstacles_row[r].push(c as i32);
                obstacles_col[c].push(r as i32);
            }
            else if *col == '^' {
                start_pos = (r as i32, c as i32);
            }
        } 
    }

    GridContext {grid: grid, obstacles_row: obstacles_row, obstacles_col: obstacles_col, start_pos: start_pos}
}

// ===========================================================================================================================================

// ========================================================== Helper Functions ===============================================================
const DIRS: [(i32,i32); 4] = [(-1,0), (0,1), (1,0), (0,-1)];
fn walk_grid(grid_context: &GridContext) -> Vec<(i32,i32)> {
    let mut cur_dir = 0;
    let mut visited: Vec<Vec<bool>> = (0..grid_context.grid.len()).map(|_| vec![false; grid_context.grid[0].len()]).collect();
    let mut visited_list: Vec<(i32,i32)> = Vec::new();

    let mut cur_r = grid_context.start_pos.0;
    let mut cur_c = grid_context.start_pos.1;

    let grid = &grid_context.grid;

    visited[grid_context.start_pos.0 as usize][grid_context.start_pos.1 as usize] = true;

    loop {
        
        let new_r = cur_r + DIRS[cur_dir].0;
        let new_c = cur_c + DIRS[cur_dir].1;
        if new_r < 0 || new_r >= grid.len() as i32 || new_c < 0 || new_c >= grid[0].len() as i32 { break; }

        if grid[new_r as usize][new_c as usize] == '#' {
            cur_dir = (cur_dir + 1) % 4;
        } else {
            cur_r = new_r;
            cur_c = new_c;
            if !visited[cur_r as usize][cur_c as usize] {
                visited[cur_r as usize][cur_c as usize] = true;
                visited_list.push((cur_r, cur_c));
            }
        }
    }

    visited_list

}

fn check_loop(grid_context: &GridContext, new_obstacle: (i32,i32)) -> i32 {
    let mut obstacles_hit : HashMap<(i32,i32), HashSet<i32>> = HashMap::new();
    let mut cur_dir: i32 = 0;

    let obstacles_row = &grid_context.obstacles_row;
    let obstacles_col = &grid_context.obstacles_col;

    let mut cur_pos = grid_context.start_pos;
    let mut obst_pos: (i32, i32);
    loop {
        match cur_dir {
            0 => {
                let mut upper_bound = obstacles_col[cur_pos.1 as usize]
                                                    .binary_search_by(|element| match element.cmp(&cur_pos.0) {
                                                        Ordering::Equal => Ordering::Greater,
                                                        ord => ord,
                                                    }).unwrap_err();
                if upper_bound > 0 { upper_bound = upper_bound - 1; }

                let mut hit_r = obstacles_col[cur_pos.1 as usize][upper_bound];
                let mut obst_found = true;
                if hit_r > cur_pos.0 { hit_r = -1000; obst_found = false; } 

                if new_obstacle.1 == cur_pos.1 && new_obstacle.0 < cur_pos.0 && new_obstacle.0 > hit_r {
                    cur_pos = (new_obstacle.0 + 1, cur_pos.1);
                    obst_pos = (new_obstacle.0, cur_pos.1);
                } else if obst_found{
                    cur_pos = (hit_r + 1, cur_pos.1);
                    obst_pos = (hit_r, cur_pos.1);
                } else {
                    return 0;
                }

            },
            1 => {
                let lower_bound = obstacles_row[cur_pos.0 as usize]
                                                    .binary_search_by(|element| match element.cmp(&cur_pos.1) {
                                                        Ordering::Equal => Ordering::Greater,
                                                        ord => ord,
                                                    }).unwrap_err();

                let hit_c: i32;
                let mut obst_found = true;
                if lower_bound >= obstacles_row[cur_pos.0 as usize].len() { 
                    hit_c = 10000000;
                    obst_found = false;
                } else {
                    hit_c = obstacles_row[cur_pos.0 as usize][lower_bound];
                }

                if new_obstacle.0 == cur_pos.0 && new_obstacle.1 > cur_pos.1 && new_obstacle.1 < hit_c {
                    cur_pos = (cur_pos.0, new_obstacle.1 - 1);
                    obst_pos = (cur_pos.0, new_obstacle.1);
                } else if obst_found {
                    cur_pos = (cur_pos.0, hit_c - 1);
                    obst_pos = (cur_pos.0, hit_c);
                } else {
                    return 0;
                }

            },
            2 => {
                let lower_bound = obstacles_col[cur_pos.1 as usize]
                                                    .binary_search_by(|element| match element.cmp(&cur_pos.0) {
                                                        Ordering::Equal => Ordering::Greater,
                                                        ord => ord,
                                                    }).unwrap_err();

                let hit_r: i32;
                let mut obst_found = true;
                if lower_bound >= obstacles_col[cur_pos.1 as usize].len() { 
                    hit_r = 10000000;
                    obst_found = false;
                } else {
                    hit_r = obstacles_col[cur_pos.1 as usize][lower_bound];
                }

                if new_obstacle.1 == cur_pos.1 && new_obstacle.0 > cur_pos.0 && new_obstacle.0 < hit_r {
                    cur_pos = (new_obstacle.0 - 1, cur_pos.1);
                    obst_pos = (new_obstacle.0, cur_pos.1);
                } else if obst_found{
                    cur_pos = (hit_r - 1, cur_pos.1);
                    obst_pos = (hit_r, cur_pos.1);
                } else {
                    return 0;
                }

            },
            3 => {
                let mut upper_bound = obstacles_row[cur_pos.0 as usize]
                                                    .binary_search_by(|element| match element.cmp(&cur_pos.1) {
                                                        Ordering::Equal => Ordering::Greater,
                                                        ord => ord,
                                                    }).unwrap_err();

                if upper_bound > 0 { upper_bound = upper_bound - 1; }

                let mut hit_c = obstacles_row[cur_pos.0 as usize][upper_bound];
                let mut obst_found = true;
                if hit_c > cur_pos.1 { hit_c = -1000; obst_found = false; } 

                if new_obstacle.0 == cur_pos.0 && new_obstacle.1 < cur_pos.1 && new_obstacle.1 > hit_c {
                    cur_pos = (cur_pos.0, new_obstacle.1 + 1);
                    obst_pos = (cur_pos.0, new_obstacle.1);
                } else if obst_found{
                    cur_pos = (cur_pos.0, hit_c + 1);
                    obst_pos = (cur_pos.0, hit_c);
                } else {
                    return 0;
                }

            },
            _ => { println!("Invalid direction"); std::process::exit(1); }
        }

        let list_set = obstacles_hit.entry(obst_pos).or_insert(HashSet::new());
        if list_set.contains(&cur_dir) {
            return 1;
        } else {
            list_set.insert(cur_dir);
        }

        cur_dir = (cur_dir + 1) % 4;
    }
}


// ===========================================================================================================================================

pub fn part1()
{
    let grid_context = read_inputs(Path::new("input/day06.txt"));
    let visited_list = walk_grid(&grid_context);
    println!("{}", visited_list.len() + 1);
}

pub fn part2()
{
    let grid_context = read_inputs(Path::new("input/day06.txt"));
    let visited_list = walk_grid(&grid_context);
    let mut ans = 0;
    for obstacle in visited_list {
        let loop_found = check_loop(&grid_context, obstacle);
        ans += loop_found;
    }

    println!("{}",ans);
}