use std::{fs::File, io::Read, path::Path};

pub fn part_1() -> u64 {
    let input = file_to_string("input/day_01.txt");
    let mut cal_vals: Vec<u64> = vec![];

    for line in input.lines() {
        let first = line
            .chars()
            .find(|c| c.is_numeric())
            .expect("find first number");
        let last = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .expect("find last number");
        cal_vals.push(
            format!("{first}{last}")
                .parse::<u64>()
                .expect("valid u64 when combining first and last"),
        )
    }
    cal_vals.into_iter().sum()
}

fn file_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).expect("valid file path");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("read file to string");
    contents
}
