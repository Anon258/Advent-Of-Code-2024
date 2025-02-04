use std::fs;
use std::path::Path;

fn read_inputs(filename: &Path) -> Vec<Vec<(i64, i64)>> {
    let content= fs::read_to_string(filename)
                            .expect("Unable to read file");

    let tests = content.split("\n\n")
                                    .map(|text| {
                                        let texts: Vec<(i64, i64)> = text.split("\n")
                                                        .map(|line| parse_line(line))
                                                        .collect();

                                        texts
                                    })
                                    .collect();
    tests
}

fn parse_line(text: &str) -> (i64, i64) {
    let (_, button) = text.split_once(": ").unwrap();
    let (a,b) = button.split_once(", ").unwrap();
    (a[2..].parse().unwrap(), b[2..].parse().unwrap())
} 


pub fn part1() {
    let tests = read_inputs(Path::new("input/day13.txt"));

    let mut ans = 0;

    for test in tests {
        let a = &test[0];
        let b = &test[1];
        let target = &test[2];

        let det = a.0 * b.1 - a.1 * b.0;
        let count = ((b.1 * target.0 - b.0 * target.1), (a.0 * target.1 - a.1 * target.0));
        if !(count.0/det < 0 || count.1/det < 0 || count.0 % det != 0 || count.1 % det != 0) {
            ans += (3 * count.0 + count.1)/det;
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let tests = read_inputs(Path::new("input/day13.txt"));

    let mut ans = 0;

    for test in tests {
        let a = &test[0];
        let b = &test[1];
        let target = (test[2].0 + 10000000000000, test[2].1 + 10000000000000);

        let det = a.0 * b.1 - a.1 * b.0;
        let count = ((b.1 * target.0 - b.0 * target.1), (a.0 * target.1 - a.1 * target.0));
        if !(count.0/det < 0 || count.1/det < 0 || count.0 % det != 0 || count.1 % det != 0) {
            ans += (3 * count.0 + count.1)/det;
        }
    }

    println!("{}", ans);
}