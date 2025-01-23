use std::fs;
use std::path::Path;

fn read_inputs(filename: &Path) -> (Vec<Vec<i32>>, (usize, usize)) {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");

    let mut grid: Vec<Vec<i32>>= content.split('\n')
                                    .map(|s| s.chars()
                                                        .map(|c| (c as i32) - ('0' as i32))
                                                        .collect::<Vec<i32>>())
                                    .collect();



    for v in grid.iter_mut() {
            v.insert(0, -1);
            v.push(-1);
    }

    let dot_arr = vec![-1;grid[0].len()];
    grid.insert(0, dot_arr.clone());
    grid.push(dot_arr.clone());

    let height = grid.len();
    let width = grid[0].len();

    (grid, (height, width))
}

const DIRS: [(i32, i32); 4] = [(1,0), (0,1), (-1,0), (0,-1)];

fn walk_graph(visited: &mut [Vec<bool>], grid: &[Vec<i32>], current_loc: (i32, i32), leafs: &mut i32) {
    if visited[current_loc.0 as usize][current_loc.1 as usize] { return; }

    visited[current_loc.0 as usize][current_loc.1 as usize] = true;

    if grid[current_loc.0 as usize][current_loc.1 as usize] == 9 {
        *leafs = *leafs + 1;
        return;
    }

    for dir in DIRS {
        let loc = (current_loc.0 + dir.0, current_loc.1 + dir.1);

        if grid[loc.0 as usize][loc.1 as usize] == grid[current_loc.0 as usize][current_loc.1 as usize] + 1 {
            walk_graph(visited, grid, loc, leafs);
        }
    }
}

fn walk_graph_rated(visited: &mut [Vec<i32>], grid: &[Vec<i32>], current_loc: (i32, i32)) -> i32 {
    if visited[current_loc.0 as usize][current_loc.1 as usize] > -1 { 
        return visited[current_loc.0 as usize][current_loc.1 as usize] 
    }

    visited[current_loc.0 as usize][current_loc.1 as usize] = 0;

    if grid[current_loc.0 as usize][current_loc.1 as usize] == 9 {
        visited[current_loc.0 as usize][current_loc.1 as usize] = 1;
        return visited[current_loc.0 as usize][current_loc.1 as usize];
    }

    for dir in DIRS {
        let loc = (current_loc.0 + dir.0, current_loc.1 + dir.1);

        if grid[loc.0 as usize][loc.1 as usize] == grid[current_loc.0 as usize][current_loc.1 as usize] + 1 {
            visited[current_loc.0 as usize][current_loc.1 as usize]  += walk_graph_rated(visited, grid, loc);
        }
    }

    return visited[current_loc.0 as usize][current_loc.1 as usize];
}

pub fn part1() {
    let (grid, (height, width)) = read_inputs(Path::new("input/day10.txt"));

    let mut ans = 0;
    for i in 1..height-1 {
        for j in 1..width-1 {
            if grid[i][j] == 0 {
                let mut visited: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width]).collect();
                walk_graph(&mut visited, &grid, (i as i32, j as i32), &mut ans);
            }
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let (grid, (height, width)) = read_inputs(Path::new("input/day10.txt"));

    let mut ans = 0;
    let mut visited: Vec<Vec<i32>> = (0..height).map(|_| vec![-1; width]).collect();
    for i in 1..height-1 {
        for j in 1..width-1 {
            if grid[i][j] == 0 {
                walk_graph_rated(&mut visited, &grid, (i as i32, j as i32));
                ans += visited[i][j];
            }
        }
    }

    println!("{}", ans);
}