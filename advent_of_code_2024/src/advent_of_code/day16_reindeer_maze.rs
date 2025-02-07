use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::path::Path;

fn read_inputs(filename: &Path) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    
    let grid : Vec<Vec<char>> = content.split("\n")
                            .map(|s| s.chars().collect())
                            .collect();

    let mut start_pos = (0,0);
    let mut end_pos = (0,0);
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' { start_pos = (i,j) }
            else if grid[i][j] == 'E' { end_pos = (i,j) }
        }
    }

    (grid, start_pos, end_pos)
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct GridVertex {
    pos: (usize, usize),
    dir: usize,
    cost: usize
}

// opposite for min Heap
impl Ord for GridVertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for GridVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl GridVertex {
    fn new(pos: (usize, usize), dir: usize, cost: usize) -> Self {
        Self { pos, dir, cost }
    }
}

const DIRS: [(i32,i32); 4] = [(0,1), (1,0), (0,-1), (-1,0)];

pub fn part1() {
    let (grid, start_pos, end_pos) = read_inputs(Path::new("input/day16.txt"));

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut ans = 0;
    queue.push(GridVertex::new(start_pos, 0, 0));

    while let Some(GridVertex {pos, dir, cost}) = queue.pop() {
        if !visited.insert((pos,dir)) { continue; }

        if pos == end_pos { 
            ans = cost;
            break; 
        }

        let new_pos = ((pos.0 as i32 + DIRS[dir].0) as usize, (pos.1 as i32 + DIRS[dir].1) as usize);
        if grid[new_pos.0][new_pos.1] != '#' { 
            queue.push(GridVertex::new(new_pos, dir, cost + 1));
        }

        for turn_dir in [(dir + 1)%4, (dir + 3)%4] {
            queue.push(GridVertex::new(pos, turn_dir, cost + 1000));
        }
    }

    println!("{}",ans);
}

pub fn part2() {
    let (grid, start_pos, end_pos) = read_inputs(Path::new("input/day16.txt"));

    let mut min_costs = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(GridVertex::new(start_pos, 0, 0));

    while let Some(GridVertex {pos, dir, cost}) = queue.pop() {
        if !visited.insert((pos,dir)) { continue; }

        min_costs.insert((pos,dir), cost);

        if pos == end_pos { continue; }

        let new_pos = ((pos.0 as i32 + DIRS[dir].0) as usize, (pos.1 as i32 + DIRS[dir].1) as usize);
        if grid[new_pos.0][new_pos.1] != '#' { 
            queue.push(GridVertex::new(new_pos, dir, cost + 1));
        }

        for turn_dir in [(dir + 1)%4, (dir + 3)%4] {
            queue.push(GridVertex::new(pos, turn_dir, cost + 1000));
        }
    }

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut total_dist = usize::MAX;

    for dir in 0..4 {
        if min_costs.contains_key(&(end_pos, dir)) {
            total_dist = total_dist.min(min_costs[&(end_pos, dir)]);
        }
    }

    for dir in 0..4 {
        if min_costs.contains_key(&(end_pos, dir)) {
            queue.push(GridVertex::new(end_pos,dir, total_dist));
        }
    }

    while let Some(GridVertex {pos, dir, cost}) = queue.pop() {
        visited.insert(pos);

        if pos == start_pos { continue; }

        let rev_dir = (dir + 2) % 4;
        let new_pos = ((pos.0 as i32 + DIRS[rev_dir].0) as usize, (pos.1 as i32 + DIRS[rev_dir].1) as usize);

        for vertex in [GridVertex::new(new_pos, dir, cost.saturating_sub(1)),
                                    GridVertex::new(pos, (dir + 1)%4, cost.saturating_sub(1000)),
                                    GridVertex::new(pos, (dir + 3)%4, cost.saturating_sub(1000))] {
            if min_costs.contains_key(&(vertex.pos, vertex.dir)) && min_costs[&(vertex.pos, vertex.dir)] == vertex.cost {
                queue.push(vertex);
                min_costs.remove(&(vertex.pos, vertex.dir));
            }
        }
    }

    println!("{}", visited.len());
}