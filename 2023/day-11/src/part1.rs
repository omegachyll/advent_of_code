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

fn xpand(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // expands in the x direction
    let mut x_expanded: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        x_expanded.push(line.to_vec());
        if !line.contains(&'#') {
            x_expanded.push(line.to_vec());
        }
    }
    x_expanded
}

fn parse_data_expand(input: &str) -> Vec<Vec<char>> {
    let in_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    transpose(xpand(transpose(xpand(in_grid))))
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = parse_data_expand(input);

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

    let result = galaxies
        .iter()
        .enumerate()
        .map(|(i, val)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|other| {
                    // info!(?val);
                    // info!(?other);
                    let cal = (other.x - val.x).abs() + (other.y - val.y).abs();
                    // info!(cal);
                    cal
                })
                .sum::<i32>()
        })
        .sum::<i32>();
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
        assert_eq!("374", process(input.as_str())?);
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
