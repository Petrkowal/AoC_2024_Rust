use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
// Try adding "+" and "*" and find out if it is possible to get the result number
// Evaluate the expression always from left to right, ignoring the operator precedence

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_possible(result: usize, operands: &[usize]) -> bool {
        for i in 0..(1 << (operands.len() - 1)) {
            let mut value = operands[0];
            for j in 0..operands.len() - 1 {
                if i & (1 << j) != 0 {
                    value += operands[j + 1];
                } else {
                    value *= operands[j + 1];
                }
            }
            if value == result {
                return true;
            }
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let equations: Vec<(usize, Vec<usize>)> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(": ");
                let result = parts.next().unwrap().parse::<usize>().unwrap();
                let operands = parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                (result, operands)
            })
            .collect();
        let mut sum = 0;

        for (result, operands) in equations {
            if is_possible(result, &operands) {
                sum += result;
            }
        }

        Ok(sum)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn concat(a: usize, b: usize) -> usize {
        let mut num = b;
        let mut len = 0;
        while num > 0 {
            num /= 10;
            len += 1;
        }
        a * 10usize.pow(len as u32) + b
    }

    fn is_possible_pt2(result: usize, operands: &[usize]) -> bool {
        let mut newoperands = operands.to_vec();
        if operands.len() == 1 {
            return operands[0] == result;
        }

        if operands[0] > result {
            return false;
        }

        let val1 = newoperands.remove(0);
        let val2 = newoperands.remove(0);

        // add
        newoperands.insert(0, val1 + val2);
        if is_possible_pt2(result, newoperands.as_slice()) {
            return true;
        }
        newoperands.remove(0);

        // mul
        newoperands.insert(0, val1 * val2);
        if is_possible_pt2(result, newoperands.as_slice()) {
            return true;
        }
        newoperands.remove(0);

        // concat
        newoperands.insert(0, concat(val1, val2));
        if is_possible_pt2(result, newoperands.as_slice()) {
            return true;
        }

        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let equations: Vec<(usize, Vec<usize>)> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(": ");
                let result = parts.next().unwrap().parse::<usize>().unwrap();
                let operands = parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                (result, operands)
            })
            .collect();
        let mut sum = 0;

        for (result, operands) in equations {
            if is_possible_pt2(result, &operands) {
                sum += result;
            }
        }

        Ok(sum)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
