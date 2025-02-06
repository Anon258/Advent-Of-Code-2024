use std::fs;
use std::path::Path;

fn read_inputs(filename: &Path) -> (Vec<Vec<char>>, Vec<usize>, (usize, usize)) {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");

    let (grid, path) = content.split_once("\n\n").unwrap();


    let grid : Vec<Vec<char>> = grid.split("\n")
                                    .map(|s| s.chars().collect())
                                    .collect();
    
    
    
    let path: Vec<usize> = path.chars()
                                                .filter(|c| *c != '\n' )
                                                .map(|c| {
                                                    match c {
                                                        '>' => 0,
                                                        '^' => 1,
                                                        '<' => 2,
                                                        'v' => 3,
                                                        _ => std::process::exit(1),
                                                    }
                                                }).collect();
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (grid, path, (i,j))
            }
        }
    }

    (grid, path, (usize::MAX,usize::MAX))
}

const DIRS: [(i32,i32); 4] = [(0,1), (-1,0), (0,-1), (1,0)];

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        print!("\n");
    }
}

fn convert_grid(grid: &mut Vec<Vec<char>>) {
    for r in grid.iter_mut() {
        *r = r.iter()
            .flat_map(|c| {
                match c {
                    '.' => ['.','.'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    '#' => ['#', '#'],
                    _ => unreachable!(),
                }
            }).collect::<Vec<char>>();
    }
}

fn move_step(current_pos: (usize, usize), dir: usize, grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    
    let mut obstacle_shift = false;
    let mut new_pos = current_pos;
    loop {
        new_pos = ((new_pos.0 as i32 + DIRS[dir].0) as usize, (new_pos.1 as i32 + DIRS[dir].1) as usize);

        if grid[new_pos.0][new_pos.1] == '.' { 
            if obstacle_shift {
                grid[new_pos.0][new_pos.1] = 'O';
                new_pos = ((current_pos.0 as i32 + DIRS[dir].0) as usize, (current_pos.1 as i32 + DIRS[dir].1) as usize);
            }

            grid[current_pos.0][current_pos.1] = '.';
            grid[new_pos.0][new_pos.1] = '@';
            return new_pos
        } else if grid[new_pos.0][new_pos.1] == '#' {
            return current_pos
        } else {
            obstacle_shift = true;
        }
    }
}

fn next_pos(pos: (usize, usize), dir: usize) -> (usize, usize) {
    ((pos.0 as i32 + DIRS[dir].0) as usize, (pos.1 as i32 + DIRS[dir].1) as usize)
}

fn check_move_possible(pos: (usize, usize), dir: usize, grid: &Vec<Vec<char>>) -> bool {

    let new_pos = next_pos(pos, dir);

    let (left, right) = match grid[new_pos.0][new_pos.1] {
        '.' => return true,
        '#' => return false,
        '[' => (new_pos, (new_pos.0, new_pos.1 + 1)),
        ']' => ((new_pos.0, new_pos.1 - 1), new_pos),
        _ => unreachable!(),
    };

    let left_pos = next_pos(left, dir);

    if DIRS[dir].0 == 0 {
        return check_move_possible(new_pos, dir, grid)
    } else if grid[left_pos.0][left_pos.1] == '[' {
        return check_move_possible(left, dir, grid)
    } else {
        return check_move_possible(left, dir, grid) 
                && check_move_possible(right, dir, grid)
    }
}

fn move_boxes(pos: (usize, usize), dir: usize, grid: &mut Vec<Vec<char>>) {

    let new_pos = next_pos(pos, dir);

    let (left, right) = match grid[new_pos.0][new_pos.1] {
        '.' => return,
        '[' => (new_pos, (new_pos.0, new_pos.1 + 1)),
        ']' => ((new_pos.0, new_pos.1 - 1), new_pos),
        _ => unreachable!(),
    };

    if DIRS[dir].0 == 0 {
        move_boxes(next_pos(new_pos, dir), dir, grid);
    } else {
        move_boxes(left, dir, grid); 
        move_boxes(right, dir, grid);
    }

    let left_pos = next_pos(left, dir);
    let right_pos = next_pos(right, dir);

    grid[left.0][left.1] = '.'; 
    grid[right.0][right.1] = '.'; 
    grid[left_pos.0][left_pos.1] = '['; 
    grid[right_pos.0][right_pos.1] = ']'; 
}

pub fn part1() {
    let (mut grid, path, mut start_pos) = read_inputs(Path::new("input/day15.txt"));
    for dir in path {
        start_pos = move_step(start_pos, dir, &mut grid);
    }

    let mut r_sum = 0;
    let mut c_sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                r_sum += i;
                c_sum += j;
            }
        }
    }

    println!("{}", 100 * r_sum + c_sum);
}

pub fn part2() {
    let (mut grid, path, mut start_pos) = read_inputs(Path::new("input/day15.txt"));
    start_pos = (start_pos.0, start_pos.1 * 2);
    convert_grid(&mut grid);
    // print_grid(&grid);

    for dir in path {
        // println!("Move: {}", dir);
        let res = check_move_possible(start_pos, dir, &grid);
        // println!("Move Possible: {}", res);
        if res {
            move_boxes(start_pos, dir, &mut grid);
            grid[start_pos.0][start_pos.1] = '.';
            start_pos = next_pos(start_pos, dir);
            grid[start_pos.0][start_pos.1] = '@';
        }
        // print_grid(&grid);
        // println!("");
    }

    // print_grid(&grid);

    let mut r_sum = 0;
    let mut c_sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                r_sum += i;
                c_sum += j;
            }
        }
    }

    println!("{}", 100 * r_sum + c_sum);
}