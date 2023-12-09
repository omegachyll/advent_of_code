use itertools::{Itertools, Position};
use std::{error::Error, fs, ops::Deref, time::Instant};
use tracing::info;

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    rank: (u32, (u32, u32, u32, u32, u32)),
}

impl<'a> Hand<'a> {
    fn init(cards: &str, bid: u32) -> Hand {
        let counts = cards.chars().counts();
        let values = if let Some(joker_count) = counts.get(&'J') {
            if *joker_count == 5 {
                "5".to_string()
            } else {
                counts
                    .iter()
                    .filter_map(|(key, value)| (key != &'J').then_some(value))
                    .sorted()
                    .with_position()
                    .map(|(position, value)| match position {
                        Position::Last | Position::Only => value + joker_count,
                        _ => *value,
                    })
                    .join("")
            }
        } else {
            counts.values().sorted().join("")
        };
        // info!(?counts);
        // info!(?values);
        let hand_type = match values.deref() {
            "5" => 6,
            "14" => 5,
            "23" => 4,
            "113" => 3,
            "122" => 2,
            "1112" => 1,
            "11111" => 0,
            _ => panic!("hand should be valid"),
        };
        let high_card = cards
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                value => value.to_digit(10).unwrap(),
            })
            .collect_tuple()
            .unwrap();

        let rank = (hand_type, high_card);

        Hand { cards, bid, rank }
    }
}

#[tracing::instrument(skip(input))]
fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let result = input
        .trim()
        .split("\n")
        .map(|hand| {
            let (cards, bid) = hand.split_once(" ").expect("this should be a valid split");
            let bid = bid.parse::<u32>().expect("should be a number");
            Hand::init(cards, bid)
            // (hand, bid.parse::<u32>().expect("should be a number"))
        })
        .sorted_by_key(|h| h.rank)
        .enumerate()
        .map(|(index, hand)| {
            info!(index);
            info!(?hand);
            (index as u32 + 1) * hand.bid
        })
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing::instrument]
    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("input.txt").expect("should be string");
        assert_eq!("252052080", process(input.as_str())?);
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
