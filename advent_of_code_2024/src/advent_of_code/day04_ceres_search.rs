use std::fs;
use std::path::Path;


// ============================================================= FILE I/O ====================================================================
fn read_inputs(filename: &Path) -> Vec<Vec<char>> {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    let mut content: Vec<Vec<char>> = content.split("\n")
                                        .map(|s| {s.chars().collect()})
                                        .collect();

    for v in content.iter_mut() {
        for _i in 0..WORD.len()-1 {
            v.insert(0, '.');
            v.push('.');
        }
    }

    let dot_arr = vec!['.';content[0].len()];
    for _i in 0..WORD.len()-1 {
        content.insert(0, dot_arr.clone());
        content.push(dot_arr.clone());
    }
    
    content
}

// ===========================================================================================================================================

// ========================================================== Helper Functions ===============================================================
const DIRS: [(i32,i32); 8] = [(1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)];
const WORD: [char; 4] = ['X','M','A','S'];

fn check_occurance(text: &Vec<Vec<char>>, dir: usize, x: i32, y:i32) -> i32 {
    let mut found = true;
    for i in 1..WORD.len() {
        let x_dir = (x + DIRS[dir].0 * (i as i32)) as usize;
        let y_dir = (y + DIRS[dir].1 * (i as i32)) as usize;
        found = found & (text[x_dir][y_dir] == WORD[i]);
    }

    found as i32
}

// ===========================================================================================================================================

pub fn part1() -> i32 {
    let res = read_inputs(Path::new("input/day04.txt"));

    let mut ans = 0;
    for i in WORD.len()-1..res.len()-(WORD.len()-1) {
        for j in WORD.len()-1..res[0].len()-(WORD.len()-1) {
            if res[i][j] == WORD[0] {
                for k in 0..8 {
                    ans += check_occurance(&res, k, i as i32, j as i32);
                }
            }
        }
    }

    ans
}

pub fn part2() -> i32 {
    let res = read_inputs(Path::new("input/day04.txt"));

    let mut ans = 0;
    for i in WORD.len()-1..res.len()-(WORD.len()-1) {
        for j in WORD.len()-1..res[0].len()-(WORD.len()-1) {
            if res[i][j] == WORD[2] {
                let diag1 = (res[i-1][j-1] == 'M' && res[i+1][j+1] == 'S') || (res[i-1][j-1] == 'S' && res[i+1][j+1] == 'M');
                let diag2 = (res[i-1][j+1] == 'M' && res[i+1][j-1] == 'S') || (res[i-1][j+1] == 'S' && res[i+1][j-1] == 'M');

                if diag1 && diag2 { ans += 1; }
            }
        }
    }

    ans
}
