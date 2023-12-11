use std::{error::Error, fs, time::Instant};
use tracing::info;

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().expect("Should be a number"))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    result.sort();
    let result = result.iter().rev().take(3).sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("24000", process(input.as_str())?);
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
