use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn read_inputs(filename: &Path) -> (Vec<Vec<char>>, (usize, usize)) {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    let grid: Vec<Vec<char>> = content.split("\n")
                                        .map(|s| {s.chars().collect()})
                                        .collect();

    let bounds = (grid.len(), grid[0].len());
    (grid, bounds)
}
const DIRS: [usize; 4] = [0,1,2,3];

fn next(dir: &usize, pos: (usize, usize)) -> (usize, usize) {
    match dir {
        0 => (pos.0, pos.1.wrapping_sub(1)),
        1 => (pos.0 + 1, pos.1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0.wrapping_sub(1), pos.1),
        _ => {
            eprintln!("ERROR: Invalid direction");
            std::process::exit(1);
        }
    }
}

fn flood_fill(visited: &mut [Vec<bool>], grid: &[Vec<char>], (height, width): (usize, usize), pos: (usize, usize)) -> (HashSet<(usize, usize)>,usize) {
    if visited[pos.0][pos.1] { return (HashSet::new(), 0) }

    let mut area: HashSet<(usize, usize)> = HashSet::new();
    area.insert(pos);
    visited[pos.0][pos.1] = true;

    let mut queue = Vec::new();
    queue.push(pos);
    let mut perimeter = 0;

    while let Some(cur) = queue.pop() {
        for next_loc in DIRS.iter().map(|d| next(d,cur) ) {
            if next_loc.0 >= height || next_loc.1 >= width { 
                perimeter += 1;
                continue; 
            }

            if grid[pos.0][pos.1] == grid[next_loc.0][next_loc.1] {
                if !visited[next_loc.0][next_loc.1] {
                    visited[next_loc.0][next_loc.1] = true;
                    area.insert(next_loc);
                    queue.push(next_loc);
                }
            } else {
                perimeter += 1;
            }
        }
    }

    return (area,perimeter);
}

fn count_corners(area: &HashSet<(usize, usize)>) -> usize {
    let mut corners = 0;
    for &point in area {
        for &dir in DIRS.iter() {
            let next_dir = (dir + 1) % 4;
            corners += (area.contains(&next(&dir, point)) 
                        && area.contains(&next(&next_dir, point))
                        && !area.contains(&next(&dir, next(&next_dir, point)))) as usize;
            
            corners += (!area.contains(&next(&dir, point)) 
                        && !area.contains(&next(&next_dir, point))) as usize;
        }
    }

    corners
}

pub fn part1() {
    let (grid, bounds) = read_inputs(Path::new("input/day12.txt"));
    let mut visited: Vec<Vec<bool>> = (0..bounds.0).map(|_| vec![false; bounds.1]).collect();

    let mut ans = 0;
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            let (area, perimeter) = flood_fill(&mut visited, &grid, bounds, (i,j));
            ans += area.len() * perimeter;
        }
    }

    println!("{}",ans);
}

pub fn part2() {
    let (grid, bounds) = read_inputs(Path::new("input/day12.txt"));
    let mut visited: Vec<Vec<bool>> = (0..bounds.0).map(|_| vec![false; bounds.1]).collect();

    let mut ans = 0;
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            let (area, _perimeter) = flood_fill(&mut visited, &grid, bounds, (i,j));
            let corners = count_corners(&area);
            ans += area.len() * corners;
        }
    }

    println!("{}",ans);
}