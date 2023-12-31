// Determine which games would have been possible if the bag had
// been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those games?

use std::{convert::Infallible, str::FromStr};

use aoc_2023_rust::util::file_to_string;

pub fn part_1() -> usize {
    let input = file_to_string("input/day_02.txt");
    let mut result = 0;
    for (i, line) in input.lines().enumerate() {
        let game = Game::from_str(line).expect("game from line");
        if game.is_valid() {
            result += i + 1;
        }
    }
    result
}

pub fn part_2() -> usize {
    let input = file_to_string("input/day_02.txt");
    let mut result = 0;
    for line in input.lines() {
        let game = Game::from_str(line).expect("game from line");
        result += game.power();
    }
    result
}

struct Game(Vec<Set>);

impl Game {
    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .all(|s| s.red < 13 && s.green < 14 && s.blue < 15)
    }

    fn power(self) -> usize {
        let min_set: Set = self
            .0
            .into_iter()
            .fold(Set::default(), |acc, s| acc.combine(s));
        min_set.power()
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sets_str = s.split(':').last().expect("split game str on :");
        Ok(Self(
            sets_str
                .split(';')
                .map(|ss| Set::from_str(ss).expect("set from string"))
                .collect(),
        ))
    }
}

#[derive(Debug, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn combine(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl FromStr for Set {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        for count_str in s.split(',').map(|s| s.trim()) {
            let mut c = count_str.split_whitespace();
            let count = c
                .next()
                .expect("count split produces number")
                .parse::<usize>()
                .expect("number is valid");
            match c
                .next()
                .expect("count split on whitespace produces a color")
            {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                invalid => panic!("invalid color: {invalid}"),
            }
        }
        Ok(set)
    }
}
