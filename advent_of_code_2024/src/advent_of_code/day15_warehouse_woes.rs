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
    convert_grid(&mut grid);
    print_grid(&grid);
}