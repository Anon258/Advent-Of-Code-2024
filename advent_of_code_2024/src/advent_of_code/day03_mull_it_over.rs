use regex::Regex;
use std::fs;
use std::path::Path;

// ============================================================= FILE I/O ====================================================================
fn read_lines(filename: &Path) -> String {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");
    content
}

// ===========================================================================================================================================

pub fn part1() -> i32 {
    let lines = read_lines(Path::new("input/day03.txt"));
    let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let instr_sum = reg.captures_iter(&lines[..]).map(|caps| {
        let (_,[num1,num2]) = caps.extract();
        num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }).fold(0, |mut x, a| {x = x + a; x});

    instr_sum
}

pub fn part2() -> i32 {
    let lines = read_lines(Path::new("input/day03.txt"));

    let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut enable = true;
    let instr_sum:i32 = reg.captures_iter(&lines[..]).map(|caps| {
        let str_match = caps.get(0).map_or("", |m| m.as_str());
        let mut a = 0;
        if str_match == "do()" {enable = true;}
        else if str_match == "don't()" {enable = false;}
        else {
            if enable {
                let num1 = caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                let num2 = caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                a = num1 * num2;
            }
        }

        a
    }).sum();

    instr_sum
}