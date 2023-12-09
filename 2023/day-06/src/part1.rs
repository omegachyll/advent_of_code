use std::{error::Error, fs, time::Instant};
use tracing::info;

#[tracing::instrument(skip(input))]
fn parse_times(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (times, distances) = input.split_once("\n").expect("should ne valid split");
    let times = times
        .strip_prefix("Time:")
        .expect("should exist")
        .trim()
        .split_ascii_whitespace()
        .map(|time| time.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .expect("should be list of numbers");
    let distances = distances
        .strip_prefix("Distance:")
        .expect("should exist")
        .trim()
        .split_ascii_whitespace()
        .map(|dist| dist.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .expect("should be list of numbers");
    info!(?times);
    info!(?distances);

    (times, distances)
}

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (times, distances) = parse_times(input);
    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, record_dist)| {
            (0..time)
                .into_iter()
                .filter_map(|speed| {
                    let traveled_dist = (time - speed) * speed;
                    (traveled_dist > record_dist).then_some(traveled_dist)
                })
                .count()
        })
        .product::<usize>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("288", process(input.as_str())?);
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
