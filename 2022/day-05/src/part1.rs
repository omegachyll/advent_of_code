use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1},
    sequence::{delimited, tuple},
    IResult,
};
use std::{
    collections::{vec_deque, HashMap, VecDeque},
    error::Error,
    fs, str,
    time::Instant,
};
use tracing::info;

#[derive(Debug)]
struct Moves {
    quant: u8,
    from: u8,
    to: u8,
}

fn parse_moves(input: &str) -> IResult<&str, (&str, &str, &str, &str, &str, &str)> {
    tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ))(input)
}

fn box_alpha_parse(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), alpha1, tag("]"))(input)
}

fn parse_data(input: &str) -> (HashMap<u8, Vec<&str>>, Vec<Moves>) {
    let (map_input, moves_input) = input.split_once("\n\n").expect("must be able to split");

    let mut map = map_input
        .lines()
        .last()
        .expect("Should exist")
        .trim()
        .split_ascii_whitespace()
        .map(|val| (val.parse::<u8>().unwrap(), Vec::new()))
        .collect::<HashMap<u8, Vec<&str>>>();

    let map_input = map_input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    for row in map_input.into_iter() {
        for (i, b) in row.into_iter().enumerate() {
            let b_trim = b.trim();

            if b_trim != "" {
                let (_, bx_char): (&str, &str) =
                    box_alpha_parse(b_trim).expect("should be a valid parse");
                map.entry(i as u8 + 1)
                    .and_modify(|stack| stack.push(bx_char));
            }
        }
    }

    let moves = moves_input
        .lines()
        .map(|line| {
            let data = parse_moves(line.trim()).expect("Should exist");
            let moves: Moves = Moves {
                quant: data.1 .1.parse::<u8>().expect("quant must parse"),
                from: data.1 .3.parse::<u8>().expect("from must parse"),
                to: data.1 .5.parse::<u8>().expect("to must parse"),
            };
            moves
        })
        .collect::<Vec<Moves>>();
    (map, moves)
}

#[tracing::instrument]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (mut map, moves) = parse_data(input);

    for mov in moves.iter() {
        for _ in 0..mov.quant {
            let stack = map.get_mut(&mov.from).expect("should exist");
            let bx = stack.pop().expect("should exist");
            map.entry(mov.to).and_modify(|stack| stack.push(bx));
        }
    }

    let mut res_vec: Vec<String> = vec![];
    for stack_num in 1..=map.len() {
        let stack = map.get(&(stack_num as u8)).expect("should exist");
        let bx = stack.last().expect("should exist");
        res_vec.push(bx.to_string())
    }

    let result = res_vec.join("");
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("CMZ", process(input.as_str())?);
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
