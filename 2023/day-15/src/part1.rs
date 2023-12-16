use std::{error::Error, fs, time::Instant};
use tracing::info;

fn calc_hash(input: &str) -> u32 {
    input.as_bytes().iter().fold(0, |acc, ch| {
        // info!(acc);
        // info!(ch);
        let mut value = *ch as u32 + acc;
        // info!(value);
        value *= 17;
        // info!(value);
        value %= 256;
        // info!(value);
        value
    })
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let result = input
        .split(",")
        .into_iter()
        .map(|item| calc_hash(item.trim()))
        .sum::<u32>();

    // let result = "";
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("1320", process(input.as_str())?);
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
