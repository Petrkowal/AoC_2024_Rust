use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
"; // TODO: Add the test input

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
        let mut count = 0;
        for line in reader.lines() {
            let mut increasing = false;
            let mut decreasing = false;
            let mut is_unsafe = false;
            let line = line?;
            let mut parts = line.split_whitespace();
            let mut levels: Vec<i32> = Vec::new();
            while let Some(part) = parts.next() {
                levels.push(part.parse().unwrap());
            }
            // print!("{:?}", levels);
            for i in 0..levels.len() - 1 {
                if levels[i] < levels[i + 1] {
                    increasing = true;
                } else if levels[i] > levels[i + 1] {
                    decreasing = true;
                }
                if (levels[i] - levels[i + 1]).abs() > 3 || levels[i] == levels[i + 1] {
                    is_unsafe = true;
                }
            }
            if increasing && decreasing {
                is_unsafe = true;
            }
            if !is_unsafe {
                count += 1;
            }
            // println!(" is {}safe", if is_unsafe { "un" } else { "" });
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
        let mut count: usize = 0;
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let mut levels: Vec<i32> = Vec::new();
            while let Some(part) = parts.next() {
                levels.push(part.parse().unwrap());
            }
            print!("{:?}", levels);

            let mut any_safe = false;
            for j in 0..levels.len() {
                let mut increasing = false;
                let mut decreasing = false;
                let mut is_unsafe = false;
                
                for i in 0..levels.len() - 1 {
                    if i == j {
                        continue
                    }
                    let mut plus_one = 0;
                    if i+1 == j {
                        plus_one = 1;
                    }
                    if (i + 1 + plus_one) >= levels.len() {
                        continue
                    }
                    if levels[i] < levels[i + 1 + plus_one] {
                        increasing = true;
                    } else if levels[i] > levels[i + 1 + plus_one] {
                        decreasing = true;
                    }
                    if (levels[i] - levels[i + 1 + plus_one]).abs() > 3 || levels[i] == levels[i + 1 + plus_one] {
                        is_unsafe = true;
                    }
                }
                if increasing && decreasing {
                    is_unsafe = true;
                }
                if !is_unsafe {
                    any_safe = true;
                    break;
                }
            }
            if any_safe {
                count += 1;
            }
            println!(" is {}safe", if any_safe { "" } else { "un" });
        }

        Ok(count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
