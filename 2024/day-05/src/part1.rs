use std::fs;
use std::{error::Error, ops::Range};

use nom::character::complete::multispace1;
use nom::sequence::tuple;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending},
    multi::many1,
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}
impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));
        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;
        destination_range.start + offset
    }
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(multispace1, complete::u64))
        .parse(input)?;
    let (input, maps) = many1(seed_map)(input)?;
    Ok((input, (seeds, maps)))
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (_, (seeds, maps)) = parse_seedmaps(input).expect("a valid parse");
    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    Ok(locations.iter().min().expect("should exist").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("35", process(input.as_str())?);
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input_test.txt").expect("should be string");
    // println!("{}", input.as_str())
    println!("{:?}", process(input.as_str()).expect("should be a string"))
}

// #[derive(Debug)]
// struct Card {
// }

// impl Card {
//     fn score(&self) -> u64 {
//         }
//     }
// }
// use rstest::rstest;

// #[rstest]
// #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
// #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
// #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
// #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
// #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
// #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
// fn line_test(#[case] line: &str, #[case] expected: u64) {
//     assert_eq!(line, "");
//     assert_eq!(expected, 0);
// }
