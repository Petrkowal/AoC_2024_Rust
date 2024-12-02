use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
";

///This example data contains six reports each containing five levels.
//
// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:
//
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?

/// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
//
// More of the above example's reports are now safe:
//
// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
// Thanks to the Problem Dampener, 4 reports are actually safe!
//
// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines().filter_map(Result::ok).map(|line|
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            ).filter(|levels|
                levels.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                    || levels.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
            ).count())
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .filter_map(Result::ok)
            .map(|line|
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>())
            .filter(|levels|
                levels.iter().enumerate().any(|(i, _)| { // Maybe... Find a way to oneliner this later
                    let mut levels = levels.clone();
                    levels.remove(i);
                    levels.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                        || levels.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
                })).count())
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
