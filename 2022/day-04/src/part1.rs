use nom::{bytes::complete::tag, character::complete, sequence::separated_pair, IResult};
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn parse_input<'a>(input: &'a str) -> Vec<((u32, u32), (u32, u32))> {
    let range_parse = |range: &'a str| -> IResult<&str, (u32, u32)> {
        separated_pair(complete::u32, tag("-"), complete::u32)(range)
    };

    let line_parse = |line: &'a str| -> (&str, ((u32, u32), (u32, u32))) {
        separated_pair(range_parse, tag(","), range_parse)(line).expect("Must parse line")
    };

    input
        .lines()
        .map(|line| {
            let (_, line_tuples) = line_parse(line);
            line_tuples
        })
        .collect::<Vec<((u32, u32), (u32, u32))>>()
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let data = parse_input(input);
    let result = data
        .iter()
        .map(|(e1, e2)| {
            if (e1.0 <= e2.0 && e1.1 >= e2.1) || (e2.0 <= e1.0 && e2.1 >= e1.1) {
                1
            } else {
                0
            }
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
