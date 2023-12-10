use aoc_2023_rust::util::file_to_string;

pub fn part_1() -> u64 {
    let input = file_to_string("input/day_01.txt");
    let mut cal_vals: Vec<u64> = vec![];

    for line in input.lines() {
        cal_vals.push(calibration_value(&line))
    }
    cal_vals.iter().sum()
}

pub fn part_2() -> u64 {
    let input = file_to_string("input/day_01.txt");
    let mut cal_vals: Vec<u64> = vec![];

    for line in input.lines().map(digitify) {
        cal_vals.push(calibration_value(&line))
    }
    cal_vals.iter().sum()
}

fn calibration_value(s: &str) -> u64 {
    let first = s
        .chars()
        .find(|c| c.is_numeric())
        .expect("find first number");
    let last = s
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .expect("find last number");
    format!("{first}{last}")
        .parse::<u64>()
        .expect("valid u64 when combining first and last")
}

fn digitify(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric() {
            result.push(c);
        } else {
            match c {
                'o' => {
                    if s[i..].starts_with("one") {
                        result.push('1');
                    }
                }
                't' => {
                    if s[i..].starts_with("two") {
                        result.push('2');
                    } else if s[i..].starts_with("three") {
                        result.push('3');
                    }
                }
                'f' => {
                    if s[i..].starts_with("four") {
                        result.push('4');
                    } else if s[i..].starts_with("five") {
                        result.push('5');
                    }
                }
                's' => {
                    if s[i..].starts_with("six") {
                        result.push('6');
                    } else if s[i..].starts_with("seven") {
                        result.push('7');
                    }
                }
                'e' => {
                    if s[i..].starts_with("eight") {
                        result.push('8');
                    }
                }
                'n' => {
                    if s[i..].starts_with("nine") {
                        result.push('9');
                    }
                }
                _ => continue,
            }
        }
    }
    result
}
