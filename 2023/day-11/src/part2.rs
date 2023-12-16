use core::cmp::{max, min};
use glam::IVec2;
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = input.len();
    let cols = input[0].len();

    let transposed: Vec<Vec<_>> = (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect();
    transposed
}

fn find_expanded_rows_cols(input: &str) -> (Vec<i64>, Vec<i64>) {
    // expands in the x direction>
    let row_expanded = parse_data(input)
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            // info!(?line);
            if !line.contains(&'#') {
                Some(i as i64)
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();
    let col_expanded = transpose(parse_data(input))
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            // info!(?line);
            if !line.contains(&'#') {
                Some(i as i64)
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();
    (row_expanded, col_expanded)
}

fn parse_data(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = parse_data(input);
    let (expanded_rows, expanded_cols) = find_expanded_rows_cols(input);

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == &'#' {
                        Some(IVec2::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<IVec2>>()
        })
        .collect::<Vec<IVec2>>();

    let expand_multiplier = 1000000;

    let result = galaxies
        .iter()
        .enumerate()
        .map(|(i, val)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|other| {
                    let row_expander = expanded_rows
                        .iter()
                        .filter_map(|ex_row| {
                            let min = min(other.y, val.y) as i64;
                            let max = max(other.y, val.y) as i64;
                            if min < *ex_row && ex_row < &max {
                                Some(1)
                            } else {
                                None
                            }
                        })
                        .sum::<i64>();
                    let col_expander = expanded_cols
                        .iter()
                        .filter_map(|ex_col| {
                            let min = min(other.x, val.x) as i64;
                            let max = max(other.x, val.x) as i64;
                            if min < *ex_col && ex_col < &max {
                                Some(1)
                            } else {
                                None
                            }
                        })
                        .sum::<i64>();
                    // info!(?val);
                    // info!(?other);
                    // info!(row_expander);
                    // info!(col_expander);
                    let cal = (other.x - val.x).abs() + (other.y - val.y).abs();
                    // info!(cal);
                    let res = cal as i64 + (col_expander + row_expander) * (expand_multiplier - 1);
                    // info!(res);
                    res
                })
                .sum::<i64>()
        })
        .sum::<i64>();
    // info!(?galaxies);

    // for line in grid.iter() {
    //     info!(?line);
    // }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("8410", process(input.as_str())?);
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
