use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\
";

// Forward
// If wall, turn right
// Count visited cells until you leave the area

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_ahead(pos: (usize, usize), dir: usize) -> (usize, usize) {
        match dir % 4 {
            0 => (pos.0.wrapping_sub(1), pos.1),
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0 + 1, pos.1),
            3 => (pos.0, pos.1.wrapping_sub(1)),
            _ => unreachable!(),
        }
    }

    fn has_loop(maze: Vec<Vec<char>>, pos: (usize, usize), dir: usize) -> bool {
        let mut dir = dir;
        let mut pos = pos;

        let mut visited: Vec<((usize, usize), usize)> = Vec::new();
        
        while true {
            let next_pos = get_ahead(pos, dir);
            if next_pos.0 >= maze[0].len() || next_pos.1 >= maze.len() {
                break;
            }
            for i in 0..visited.len() {
                if visited[i] == (pos, dir) {
                    return true;
                }
            }
            visited.push((pos, dir));
            if maze[next_pos.0][next_pos.1] == '#' {
                // Turn right
                dir = (dir + 1) % 4;
                continue;
            }

            pos = next_pos;
            // check if pos in bounds
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let maze: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|l| l.chars().collect())
            .collect();
        // pos = '^'
        // dir = 'N'

        let mut pos: (usize, usize) = (10000, 10000);
        for i in 0..maze.len() {
            for j in 0..maze[0].len() {
                if maze[i][j] == '^' {
                    pos = (i, j);
                }
            }
        }

        let mut dir = 0;
        let mut visited: Vec<Vec<bool>> = Vec::new();
        for _ in 0..maze.len() {
            visited.push(vec![false; maze[0].len()]);
        }
        visited[pos.0][pos.1] = true;

        while true {
            visited[pos.0][pos.1] = true;
            let next_pos = get_ahead(pos, dir);
            if next_pos.0 < 0
                || next_pos.0 >= maze[0].len()
                || next_pos.1 < 0
                || next_pos.1 >= maze.len()
            {
                break;
            }
            if maze[next_pos.0][next_pos.1] == '#' {
                // Turn right
                dir = (dir + 1) % 4;
                continue;
            }
            pos = next_pos;
            // check if pos in bounds
        }

        let mut count = 0;
        for v_line in visited {
            for bool in v_line {
                if bool {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let maze: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|l| l.chars().collect())
            .collect();
        // pos = '^'
        // dir = 'N'

        let mut pos: (usize, usize) = (10000, 10000);
        for i in 0..maze.len() {
            for j in 0..maze[0].len() {
                if maze[i][j] == '^' {
                    pos = (i, j);
                }
            }
        }

        let dir = 0;
        let mut count = 0;
        for i in 0..maze.len() {
            for j in 0..maze[i].len() {
                let mut maze_copy = maze.clone();
                if maze_copy[i][j] == '#' || maze_copy[i][j] == '^' {
                    continue;
                }
                maze_copy[i][j] = '#';
                let done_count = i * maze[i].len() + j;
                let total_count = maze.len() * maze[0].len();
                println!("Progress: {}/{}", done_count, total_count);
                if has_loop(maze_copy, pos, dir) {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
