use itertools::Itertools;
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn parser(input: &str) -> Vec<(&str, &str, &str)> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect_tuple::<(&str, &str, &str)>().unwrap())
        .collect::<Vec<(&str, &str, &str)>>()
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
        .map(|(e1, e2, e3)| {
            let common = e1
                .chars()
                .find(|c| e2.contains(*c) && e3.contains(*c))
                .unwrap();
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
        assert_eq!("70", process(input.as_str())?);
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
