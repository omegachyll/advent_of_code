use std::collections::BTreeMap;
use std::result::Result;
use std::error::Error;
use nom::IResult;
use nom::multi::separated_list1;
use nom::character::complete::{alpha1, digit1, line_ending};
use std::fs;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>
}

impl<'a> Game<'a> {
    fn valid_game(&self, map: &BTreeMap<&str, u32>) -> Option<u32>{
        self.rounds
            .iter()
            .all(|round| {
           round
               .iter()
               .all(|shown_cube| {
               shown_cube.amount
                   <= *map
                   .get(shown_cube.color)
                   .expect("a valid cube")
           })
        }).then_some(self.id.parse::<u32>()
            .expect("game id should be parsable u32"))
    }
}

// 4 red
fn cude(input: &str) -> IResult<&str, Cube> {
   let(input, (amount, color)) =
    separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube {
        color, amount
    }))
}

// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cude)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) =
        preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game {id, rounds}))
}
fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) =
        separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}
fn process(
    input: &str,
    map: BTreeMap<&str, u32>
) -> Result<String, Box<dyn Error>> {
    let games = parse_games(input).expect("should parse");
    Ok(games.1.iter().filter_map(|game|game.valid_game(&map)).sum::<u32>().to_string())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let map = BTreeMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input, map)?);
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input1.txt")
        .expect("should be string");
    let map = BTreeMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    println!("{:?}", process(input.as_str(), map).expect("should be a string"))
}