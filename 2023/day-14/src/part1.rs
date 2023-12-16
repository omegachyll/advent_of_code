use std::{error::Error, fs, str::from_utf8, time::Instant};
use tracing::info;

fn transpose_grid(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = input.len();
    let cols = input[0].len();

    let transposed: Vec<Vec<_>> = (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect();
    transposed
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let sorted_grid = transpose_grid(
        transpose_grid(grid)
            .iter()
            .map(|row| {
                let joined: String = row.iter().collect();
                let substrings = joined.split("#").collect::<Vec<_>>();

                let sorted_substrings = substrings
                    .iter()
                    .map(|substring| {
                        let mut sort_substring = substring.to_string().into_bytes().clone();
                        sort_substring.sort();
                        sort_substring.reverse();
                        String::from_utf8(sort_substring).expect("should be valid")
                    })
                    .collect::<Vec<String>>();
                sorted_substrings.join("#").chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
    );

    // for i in sorted_grid.iter() {
    //     info!(?i);
    // }

    let result = sorted_grid
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let counts = row
                .iter()
                .fold(0, |acc, ch| if ch == &'O' { acc + 1 } else { acc });

            (counts * (i + 1)) as u32
        })
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("136", process(input.as_str())?);
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
