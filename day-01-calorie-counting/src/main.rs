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
    let mut sums: Vec<i32> = groups
        .into_iter()
        .map(|group| {
            group
                .split(LINE_SEPARATOR)
                .into_iter()
                .map(|line| {
                    line.parse::<i32>()
                        .expect(&format!("Couldn't parse value '{line}'"))
                })
                .sum::<i32>()
        })
        .collect();

    sums.sort();
    sums.reverse();

    let max = sums.first().expect("Couldn't determine the maximum value");
    let top_three_sum: i32 = (&sums[..3]).into_iter().sum();

    println!("The Elf carrying the most Calories has {max} Calories");
    println!("The top three Elves are carrying a total of {top_three_sum} Calories");
}
