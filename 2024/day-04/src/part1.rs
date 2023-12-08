use std::collections::HashSet;
use std::error::Error;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let power = self.winning_numbers.intersection(&self.my_numbers).count() as u32;
        if power > 0 {
            2u32.pow(power - 1)
        } else {
            0
        }
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            dbg!(acc)
        },
    )(input)
}
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_numbers, my_numbers)| Card {
            winning_numbers,
            my_numbers,
        })
        .parse(input)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (_, card_data) = dbg!(cards(input).expect("a valid parse"));
    let result = card_data.iter().map(|card| card.score()).sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        let (input, card) = card(line).expect("should be a valid card");
        assert_eq!(input, "");
        assert_eq!(expected, card.score());
    }
    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("13", process(input.as_str())?);
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("should be string");
    // println!("{}", input.as_str())
    println!("{:?}", process(input.as_str()).expect("should be a string"))
}
