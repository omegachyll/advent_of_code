use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use num::integer;
use std::{collections::BTreeMap, error::Error, fs, time::Instant};
use tracing::info;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[tracing::instrument(skip(input))]
fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;

    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;
    Ok((input, (instructions, map)))
}

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (input, (instructions, map)) = parser(input).expect("should return valid parse");
    debug_assert_eq!(input, "");

    // info!(?instructions);
    // info!(?map);

    let start_nodes = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect::<Vec<&str>>();
    info!(?start_nodes);

    let step_count = start_nodes
        .iter()
        .map(|start| {
            let mut current_node = *start;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = map.get(current_node).expect("must always exist");
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };
                    if next_node.ends_with("Z") {
                        Some((index + 1) as u64)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
                .expect("infinite iteratoe must produce output")
        })
        .fold(1, |acc: u64, item| integer::lcm(acc, item));

    info!(?step_count);

    let result = step_count;
    Ok(result.to_string())
}

//11A - 2 22A - 3
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::try_init();
        let input_file = "input_test_part2.txt";
        let input = fs::read_to_string(input_file.to_string()).expect("should be string");
        // info!(input);
        assert_eq!("6", process(input.as_str())?);
        Ok(())
    }
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("should be string");
    // info!(input);
    println!("{:?}", process(input.as_str()).expect("should be a string"));
    let elapsed = now.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
