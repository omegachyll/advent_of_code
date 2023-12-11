use nom::{
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn match_play_options(input: &str) -> u8 {
    match input {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("must alwasy be a valid play"),
    }
}

fn parser(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(line_ending, separated_pair(alpha1, space1, alpha1))(input)
}

fn game(play: (&str, &str)) -> (u32, u32) {
    // r vs p >1 vs 2  > 1 % 3 vs 2-1 > 1 vs 1 > right wins
    // s vs p >3 vs 2  > 3 % 3 vs 2-1 > 0 vs 1 > right wins
    // r vs s > 1 vs 3 > 1 % 3 vs 3-1 > 1 vs 2 > lef wins
    let play_vals = (match_play_options(play.0), match_play_options(play.1));
    if play_vals.0 % 3 == (play_vals.1 - 1) {
        (play_vals.0 as u32, (play_vals.1 + 6) as u32)
    } else if play_vals.0 == play_vals.1 {
        ((play_vals.0 + 3) as u32, (play_vals.1 + 3) as u32)
    } else {
        ((play_vals.0 + 6) as u32, play_vals.1 as u32)
    }
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (_, games) = parser(input).expect("should be a valid parse");
    let result = games
        .iter()
        .map(|play| {
            let res = game(*play).1;
            res
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
        let input = fs::read_to_string("input_test.txt").expect("should be string");
        assert_eq!("15", process(input.as_str())?);
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
