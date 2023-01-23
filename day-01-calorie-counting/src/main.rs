// https://adventofcode.com/2022/day/1

use std::fs;

const INPUT_FILE: &str = "input.txt";

#[cfg(windows)]
const LINE_SEPARATOR: &str = "\r\n";
#[cfg(not(windows))]
const LINE_SEPARATOR: &str = "\n";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect(&format!("Cannot read {INPUT_FILE} file"));
    let blank_line = LINE_SEPARATOR.to_string() + LINE_SEPARATOR;
    let groups = input.split(&blank_line);
    let sums = groups.into_iter().map(|group| {
        group
            .split(LINE_SEPARATOR)
            .into_iter()
            .map(|line| {
                line.parse::<i32>()
                    .expect(&format!("Couldn't parse value '{line}'"))
            })
            .sum::<i32>()
    });
    let max = sums.max().expect("Couldn't determine the maximum value");

    println!("The Elf carrying the most Calories has {max} Calories");
}
