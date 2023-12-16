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

fn tilt(grid: Vec<Vec<char>>, transpose: bool, reverse: bool) -> Vec<Vec<char>> {
    let mut tilt_grid = grid.clone();
    if transpose {
        tilt_grid = transpose_grid(grid.clone());
    }
    tilt_grid = tilt_grid
        .iter()
        .map(|row| {
            let joined: String = row.iter().collect();
            let substrings = joined.split("#").collect::<Vec<_>>();
            let sorted_substrings = substrings
                .iter()
                .map(|substring| {
                    let mut sort_substring = substring.to_string().into_bytes().clone();
                    sort_substring.sort();
                    if reverse {
                        sort_substring.reverse();
                    }
                    String::from_utf8(sort_substring).expect("should be valid")
                })
                .collect::<Vec<String>>();
            sorted_substrings.join("#").chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    if transpose {
        tilt_grid = transpose_grid(tilt_grid);
    }
    tilt_grid
}

fn spin_cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = tilt(grid, true, true);
    let west = tilt(north, false, true);
    let south = tilt(west, true, false);
    let east = tilt(south, false, false);
    east
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // for i in grid.iter() {
    //     info!(?i);
    // }
    //
    // info!("****************************************************************************");

    let mut spins: Vec<Vec<Vec<char>>> = Vec::new();
    let mut count = 0;
    let mut location = 0;

    spins.push(grid.clone());

    loop {
        let start = spins.last().clone().expect("should exist");
        let spin = spin_cycle(start.to_vec());

        count += 1;
        if spins.contains(&spin) {
            for i in 0..spins.len() {
                if spins[i] == spin {
                    location = i;
                    break;
                }
            }
            break;
        }
        spins.push(spin.clone());
    }

    info!(count);
    info!(location);

    let position = (1000000000 - location) % (count - location) + location as usize;

    let final_grid = &spins[position];

    // for i in final_grid.iter() {
    //     info!(?i);
    // }

    let result = final_grid
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
    fn test_spin() -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::try_init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        let output = fs::read_to_string("2cycle_output.txt").expect("should be string");
        let output_grid = output
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(output_grid, spin_cycle(spin_cycle(grid)));
        Ok(())
    }

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::try_init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("64", process(input.as_str())?);
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
