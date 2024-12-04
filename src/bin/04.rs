use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");


    fn search(word: &str, grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let len = word.len();
        let (width, height) = (grid[0].len() as isize, grid.len() as isize);

        if (0..len).any(|i| {
            let nx = x as isize + dx * i as isize;
            let ny = y as isize + dy * i as isize;
            nx < 0 || nx >= width || ny < 0 || ny >= height
        }) {
            return false;
        }

        let found: String = (0..len)
            .map(|i| {
                let nx = (x as isize + dx * i as isize) as usize;
                let ny = (y as isize + dy * i as isize) as usize;
                grid[ny][nx]
            })
            .collect();

        found == word
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

        let patterns = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        let mut sum: u32 = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                for (dx, dy) in patterns.iter() {
                    sum += search("XMAS", &grid, x, y, *dx, *dy) as u32;
                }
            }
        }

        Ok(sum as usize)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

        let mut sum: u32 = 0;
        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                if grid[y][x] != 'A' {
                    continue;
                }

                let tl = grid[y - 1][x - 1];
                let tr = grid[y - 1][x + 1];
                let bl = grid[y + 1][x - 1];
                let br = grid[y + 1][x + 1];

                if ((tl == 'M' || tl == 'S') && (br == 'M' || br == 'S') && tl != br)
                    && ((tr == 'M' || tr == 'S') && (bl == 'M' || bl == 'S') && tr != bl)
                {
                    sum += 1;
                }
            }
        }
        Ok(sum as usize)
    }
    
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
