use std::{error::Error, fs, time::Instant};
use tracing::info;

#[tracing::instrument(skip(input))]
fn parse_times(input: &str) -> (u64, u64) {
    let (time, distance) = input.split_once("\n").expect("should ne valid split");
    let time = time
        .strip_prefix("Time:")
        .expect("should exist")
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .expect("shoud be a number");
    let distance = distance
        .strip_prefix("Distance:")
        .expect("should exist")
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .expect("shoud be a number");
    (time, distance)
}

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (time, record_dist) = parse_times(input);
    let result = (0..time)
        .into_iter()
        .filter_map(|speed| {
            let traveled_dist = (time - speed) * speed;
            (traveled_dist > record_dist).then_some(traveled_dist)
        })
        .count();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("71503", process(input.as_str())?);
        Ok(())
    }
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let now = Instant::now();
    let input = fs::read_to_string("input1.txt").expect("should be string");
    info!(input);
    println!("{:?}", process(input.as_str()).expect("should be a string"));
    let elapsed = now.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
