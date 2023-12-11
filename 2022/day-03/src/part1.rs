use std::{char, error::Error, fs, time::Instant};
use tracing::info;

fn parser(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<(&str, &str)>>()
}

fn get_priority(input: u32) -> u32 {
    info!(?input);
    let ascii = input;
    if ascii > 96 {
        ascii - 96
    } else {
        ascii - 38
    }
}

#[tracing::instrument]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let rucksacks = parser(input);

    let result = rucksacks
        .iter()
        .map(|(c1, c2)| {
            let common = c1.chars().find(|c| c2.contains(*c)).unwrap();
            dbg!(get_priority(common as u32))
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
        assert_eq!("157", process(input.as_str())?);
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
