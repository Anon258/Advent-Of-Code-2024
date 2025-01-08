use std::fs;
use std::path::Path;


// ============================================================= FILE I/O ====================================================================
fn read_inputs(filename: &Path) -> Vec<Vec<i32>> {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    let content: Vec<Vec<i32>> = content.split("\n")
                                        .map(|s| 
                                                    s.split(' ')
                                                    .map(|st| st.parse().unwrap())
                                                    .collect())
                                        .collect();

    content
}

// ===========================================================================================================================================

// ========================================================== Helper Functions ===============================================================
fn safe_dampner(arr: &[i32]) -> bool {
    let mut one_miss = true;
    let mut good = true;

    let mut prev = arr[0];
    let mut last_diff = 0;

    for i in 1..arr.len() {
        if !good {break;}

        let new_diff = arr[i] - prev;
        if i > 1 && new_diff ^ last_diff < 0 {
            good = false;
        } else if new_diff.abs() < 1 || new_diff.abs() > 3 {
            good = false;
        } else {}

        if one_miss && !good {
            one_miss = false;
            good = true;
        } else {
            prev = arr[i];
            last_diff = new_diff;
        }
    }
    good

}

// ===========================================================================================================================================

pub fn part1() -> i32 {
    let res = read_inputs(Path::new("input/day02"));

    let diff: Vec<Vec<i32>> = res.iter().map(|v: &Vec<i32>| {
            v.windows(2)
            .map(|vs| {
                let [x,y] = vs else { unreachable!() };
                y - x})
            .collect::<Vec<_>>()})
        .collect();

    let goods = diff.iter().map(|v: &Vec<i32>| {
        (v.iter().all(|s| *s >= 1 && *s <= 3)) || (v.iter().all(|s| *s <= -1 && *s >= -3))
    }).filter(|b| *b).count();

    goods as i32
}

pub fn part2() -> i32{
    let mut res: Vec<Vec<i32>> = read_inputs(Path::new("input/day02"));

    let diff = res.iter_mut().map(|v: &mut Vec<i32>| {
            let a = safe_dampner(v);
            v.reverse();
            let b = safe_dampner(v);
            a || b
        })
        .filter(|b| *b)
        .count();

    diff as i32
}