use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3\
";

fn main() -> Result<()> {
    start_day(DAY);

    fn load_data<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>)> {
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            left_list.push(parts.next().unwrap().parse().unwrap());
            right_list.push(parts.next().unwrap().parse().unwrap());
        }

        Ok((left_list, right_list))
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left_list, mut right_list) = load_data(reader)?;

        left_list.sort();
        right_list.sort();

        Ok(left_list.iter().zip(right_list).map(|(left, right)| if left > &right { left - right } else { right - left }).sum())
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left_list, right_list) = load_data(reader)?;


        Ok(left_list.iter().map(|left| left * right_list.iter().filter(|&&right| right == *left).count()).sum())
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
