use core::cmp::min;
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn transpose_grid(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = input.len();
    let cols = input[0].len();

    let transposed: Vec<Vec<_>> = (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect();
    transposed
}

fn find_horizontal_reflections(input: &str, transpose: bool) -> u32 {
    let mut pattern = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    if transpose {
        pattern = transpose_grid(pattern);
    }

    for r in 1..pattern.len() {
        let mut above = Vec::new();
        above.extend_from_slice(&pattern[..r]);
        above.reverse();
        let below = &pattern[r..];
        let range = min(below.len(), above.len());

        let match_criteria = above[..range]
            .into_iter()
            .zip(below[..range].into_iter())
            .map(|(x, y)| {
                x.into_iter()
                    .zip(y.into_iter())
                    .map(|(a, b)| if a == b { 0 } else { 1 })
                    .sum::<u32>()
            })
            .sum::<u32>();
        info!(match_criteria);
        if match_criteria == 1 {
            return r as u32;
        }
    }

    return 0;
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let patterns = input.split("\n\n").collect::<Vec<&str>>();

    // info!("{}", patterns[0]);

    let horizontal_results = patterns
        .iter()
        .map(|pattern| find_horizontal_reflections(pattern, false))
        .sum::<u32>();

    let vertical_results = patterns
        .iter()
        .map(|pattern| find_horizontal_reflections(pattern, true))
        .sum::<u32>();

    let result = vertical_results + horizontal_results * 100;

    info!(result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("400", process(input.as_str())?);
        Ok(())
    }
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("should be string");
    println!("{:?}", process(input.as_str()).expect("should be a string"));
    let elapsed = now.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
