use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                re.captures_iter(&line)
                    .map(|cap| {
                        let x: usize = cap[1].parse().unwrap();
                        let y: usize = cap[2].parse().unwrap();
                        x * y
                    })
                    .sum::<usize>()
            })
            .sum::<usize>())
    }
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        let input = reader.lines().flatten().collect::<String>();

        let mut matches = vec![];
        matches.extend(mul_re.captures_iter(&input).map(|cap| (cap.get(0).unwrap().start(), "mul", cap)));
        matches.extend(do_re.captures_iter(&input).map(|cap| (cap.get(0).unwrap().start(), "do", cap)));
        matches.extend(dont_re.captures_iter(&input).map(|cap| (cap.get(0).unwrap().start(), "don't", cap)));

        matches.sort_by_key(|(pos, _, _)| *pos);

        let mut mul_enabled = true;
        let sum: usize = matches.into_iter().fold(0, |acc, (_, kind, cap)| {
            match kind {
                "mul" if mul_enabled => acc + cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap(),
                "do" => { mul_enabled = true; acc },
                "don't" => { mul_enabled = false; acc },
                _ => acc,
            }
        });

        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
