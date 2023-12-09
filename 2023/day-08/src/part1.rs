use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
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
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
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

    let mut current_node = "AAA";

    let Some(step_count) =
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
                if next_node == "ZZZ" {
                    Some(index + 1)
                } else {
                    current_node = next_node;
                    None
                }
            })
    else {
        panic!("infinite iterator cannot produce none")
    };

    let result = step_count;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2", "input_test1.txt")]
    #[case("6", "input_test2.txt")]
    fn test_process(#[case] restult: &str, #[case] input_file: &str) -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::try_init();
        let input = fs::read_to_string(input_file.to_string()).expect("should be string");
        assert_eq!(restult, process(input.as_str())?);
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
