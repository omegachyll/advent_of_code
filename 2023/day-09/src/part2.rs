use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), complete::i32)(input)
}

#[tracing::instrument(skip(values))]
fn reduction(values: Vec<i32>) -> Vec<i32> {
    let result = values
        .windows(2)
        .map(|items| items[1] - items[0])
        .collect::<Vec<i32>>();
    result
}

#[tracing::instrument(skip(readings))]
fn extrapolate(readings: Vec<i32>) -> i32 {
    let mut start_numbers: Vec<i32> = vec![];
    start_numbers.push(readings.first().expect("must exist").clone());
    let mut reduced_readings = reduction(readings);
    loop {
        start_numbers.push(*reduced_readings.first().expect("must exist"));
        reduced_readings = reduction(reduced_readings);
        if reduced_readings.iter().sum::<i32>() == 0
            && reduced_readings.iter().product::<i32>() == 0
        {
            break;
        }
    }

    start_numbers.iter().rev().fold(0, |acc, num| num - acc)
}

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let all_sensors = input
        .lines()
        .map(|line| {
            let (_, readings) = parse_line(line).expect("should be a vec of numbers");
            readings
        })
        .collect::<Vec<Vec<i32>>>();

    let result = all_sensors
        .iter()
        .map(|readings| extrapolate(readings.to_vec()))
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("2", process(input.as_str())?);
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
