use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines();
        let mut rules = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read line");
            if line.is_empty() {
                break;
            }
            let mut parts = line.split('|');
            let rule = (
                parts
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Failed to parse rule A"),
                parts
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Failed to parse rule B"),
            );
            rules.push(rule);
        }

        let mut sum = 0;
        while let Some(line) = lines.next() {
            let mut parts: Vec<usize> = line?
                .split(',')
                .map(|p| p.parse::<usize>().expect("Failed to parse part"))
                .collect();
            let mut valid = true;
            let middle_part = &parts[parts.len() / 2];
            'for_rule: for rule in &rules {
                if !parts.contains(&rule.0) || !parts.contains(&rule.1) {
                    continue;
                }
                for i in 0..parts.len() {
                    if parts[i] == rule.1 {
                        for j in i + 1..parts.len() {
                            if parts[j] == rule.0 {
                                valid = false;
                                break 'for_rule;
                            }
                        }
                    }
                }
            }
            if valid {
                sum += middle_part;
            }
        }
        Ok(sum)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines();
        let mut rules = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read line");
            if line.is_empty() {
                break;
            }
            let mut parts = line.split('|');
            let rule = (
                parts
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Failed to parse rule A"),
                parts
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Failed to parse rule B"),
            );
            rules.push(rule);
        }

        let mut sum = 0;
        while let Some(line) = lines.next() {
            let mut parts: Vec<usize> = line?
                .split(',')
                .map(|p| p.parse::<usize>().expect("Failed to parse part"))
                .collect();
            let mut valid = true;
            'for_rule: for rule in &rules {
                if !parts.contains(&rule.0) || !parts.contains(&rule.1) {
                    continue;
                }
                for i in 0..parts.len() {
                    if parts[i] == rule.1 {
                        for j in i + 1..parts.len() {
                            if parts[j] == rule.0 {
                                valid = false;
                                break 'for_rule;
                            }
                        }
                    }
                }
            }
            if valid {
                continue;
            }

            // ticket is invalid
            // Get the correct order according to rules
            let mut used_rules: Vec<_> = Vec::new();
            for rule in &rules {
                if parts.contains(&rule.0) && parts.contains(&rule.1) {
                    used_rules.push(rule);
                }
            }
            
            // for each part with each part, if it is not ordered correctly, swap them
            for i in 0..parts.len() {
                for j in i + 1..parts.len() {
                    let mut valid = true;
                    for rule in &used_rules {
                        if parts[i] == rule.1 && parts[j] == rule.0 {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        continue;
                    }
                    parts.swap(i, j);
                }
            }
            
            let middle_part = &parts[parts.len() / 2];
            sum += middle_part;
        }
        Ok(sum)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
