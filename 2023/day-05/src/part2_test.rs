#![allow(dead_code)] // want to keep prior solutions around

use std::{num::ParseIntError, ops::Range, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct RangeMap {
    dest_start: usize,
    origin_start: usize,
    length: usize,
}

impl FromStr for RangeMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| RangeMap {
                dest_start: v[0],
                origin_start: v[1],
                length: v[2],
            })
    }
}

impl RangeMap {
    fn in_source(&self, n: usize) -> bool {
        n >= self.origin_start && n < self.origin_start + self.length
    }

    fn in_dest(&self, n: usize) -> bool {
        n >= self.dest_start && n < self.dest_start + self.length
    }

    fn try_map(&self, n: usize) -> Option<usize> {
        if self.in_source(n) {
            Some(n - self.origin_start + self.dest_start)
        } else {
            None
        }
    }

    fn try_map_back(&self, n: usize) -> Option<usize> {
        if self.in_dest(n) {
            Some(n - self.dest_start + self.origin_start)
        } else {
            None
        }
    }

    // map any part of the range that can, and return the 0, 1, or 2 ranges which still need to be mapped
    fn map_subrange(&self, r: Range<usize>) -> (Option<Range<usize>>, Vec<Range<usize>>) {
        let mapped_start = self.try_map(r.start);
        let mapped_end = self.try_map(r.end - 1).map(|e| e + 1);
        let mapped = match (mapped_start, mapped_end) {
            (Some(s), Some(e)) => Some(s..e),
            (Some(s), None) => Some(s..self.dest_start + self.length),
            (None, Some(e)) => Some(self.dest_start..e),
            (None, None) => None,
        };

        let mut remaining = Vec::new();
        if mapped.is_some() {
            if self.origin_start > r.start {
                remaining.push(r.start..self.origin_start);
            }
            if self.origin_start + self.length < r.end {
                remaining.push(self.origin_start + self.length..r.end);
            }
        }

        (mapped, remaining)
    }
}

#[derive(Debug)]
struct Mapping {
    name: String,
    maps: Vec<RangeMap>,
}

impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, maps) = s.split_once(':').expect("Expected colon");
        let name = name.trim().to_string();
        let mut maps: Vec<_> = maps
            .trim()
            .lines()
            .map(|l| l.parse::<RangeMap>())
            .collect::<Result<_, _>>()?;

        // CRUCIAL: sort by destination start
        // this will ensure that outputs are yielded in order
        maps.sort();

        Ok(Mapping { name, maps })
    }
}

impl Mapping {
    fn map(&self, n: usize) -> usize {
        self.maps.iter().find_map(|m| m.try_map(n)).unwrap_or(n)
    }

    fn map_back(&self, n: usize) -> usize {
        self.maps
            .iter()
            .find_map(|m| m.try_map_back(n))
            .unwrap_or(n)
    }

    fn outputs(&self) -> impl Iterator<Item = usize> + '_ {
        self.maps
            .iter()
            .flat_map(|m| m.dest_start..m.dest_start + m.length)
    }

    fn map_range(&self, r: Range<usize>) -> Vec<Range<usize>> {
        let mut mapped = Vec::new();

        let mut remaining = vec![r];
        while let Some(r) = remaining.pop() {
            let mut did_something = false;
            for m in &self.maps {
                if let (Some(res), rem) = m.map_subrange(r.clone()) {
                    mapped.push(res);
                    remaining.extend(rem);
                    did_something = true;
                    break;
                }
            }

            // identity mapping
            if !did_something {
                mapped.push(r);
            }
        }

        mapped
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Mapping>) {
    let (seeds, mappings) = input.split_once("\n\n").expect("Expected two sections");
    let seeds = seeds
        .trim()
        .strip_prefix("seeds: ")
        .and_then(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<usize>())
                .collect::<Result<_, _>>()
                .ok()
        })
        .expect("Expected seed numbers");

    let mappings = mappings
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Mapping>())
        .collect::<Result<_, _>>()
        .expect("Expected mappings");

    (seeds, mappings)
}

// naive forward search from seeds
fn forward(seeds: &[usize], mappings: &[Mapping]) -> usize {
    seeds
        .iter()
        .map(|s| mappings.iter().fold(*s, |s, m| m.map(s)))
        .min()
        .unwrap()
}

// Search backwards from outputs to seeds
// For each output of each layer of mapping
//   compute the location this would lead to
//   if that location is smaller than the smallest location we've found so far
//     compute backwards to what seed would get us here
//     if that seed is in the set of seeds
//       update smallest location
fn backwards_search<P>(seed_pred: P, mappings: &[Mapping]) -> usize
where
    P: Fn(usize) -> bool,
{
    let mappings_rev = mappings.iter().rev().collect::<Vec<_>>();
    let n = mappings_rev.len();

    let mut smallest = usize::MAX;
    for (i, mapping) in mappings_rev.iter().enumerate() {
        let s = mapping
            .outputs()
            .find_map(|o| {
                let maybe_seed = mappings_rev[i..].iter().fold(o, |s, m| m.map_back(s));

                if seed_pred(maybe_seed) {
                    let resultant_location = mappings[n - i..].iter().fold(o, |s, m| m.map(s));
                    Some(resultant_location)
                } else {
                    None
                }
            })
            .unwrap_or(usize::MAX);

        smallest = smallest.min(s);
    }
    smallest
}

// forward search but using ranges of seeds to represent contiguous regions
// this is faster than either of the above for part 2
fn ranges_forward(seeds: &[Range<usize>], mappings: &[Mapping]) -> usize {
    let mut current = seeds.to_vec();
    for m in mappings {
        current = current
            .iter()
            .flat_map(|r| m.map_range(r.clone()))
            .collect();
    }

    current.iter().min_by_key(|r| r.start).unwrap().start
}

fn part_1_naive(seeds: &[usize], mappings: &[Mapping]) -> usize {
    forward(seeds, mappings)
}

fn part_1_backward(seeds: &[usize], mappings: &[Mapping]) -> usize {
    // works, but slower than needed
    let seeds = std::collections::HashSet::<usize>::from_iter(seeds.iter().copied());
    backwards_search(|u| seeds.contains(&u), mappings)
}

fn part_1_ranges(seeds: &[usize], mappings: &[Mapping]) -> usize {
    let seeds = seeds.iter().map(|s| *s..*s + 1).collect::<Vec<_>>();
    ranges_forward(&seeds, mappings)
}

fn rangeify_seeds(seeds: &[usize]) -> Vec<Range<usize>> {
    seeds
        .chunks_exact(2)
        .map(|c| {
            let start = c[0];
            let len = c[1];
            start..start + len
        })
        .collect()
}

fn part_2_backward(seeds: &[usize], mappings: &[Mapping]) -> usize {
    let new_seeds = rangeify_seeds(seeds);

    backwards_search(|u| new_seeds.iter().any(|r| r.contains(&u)), mappings)
}

fn part_2_ranges(seeds: &[usize], mappings: &[Mapping]) -> usize {
    let new_seeds = rangeify_seeds(seeds);

    ranges_forward(&new_seeds, mappings)
}

fn solve_day_05(input: &str) -> (usize, usize) {
    let (seeds, mappings) = parse_input(input);

    use std::time::Instant;
    let now = Instant::now();

    let part_1 = part_1_naive(&seeds, &mappings);

    let elapsed = now.elapsed();
    println!("Elapsed (part 1): {:.2?}", elapsed);

    let now = Instant::now();

    let part_2 = part_2_ranges(&seeds, &mappings);

    let elapsed = now.elapsed();
    println!("Elapsed (part 2): {:.2?}", elapsed);

    (part_1, part_2)
}

fn main() {
    let input = include_str!("../input1.txt");

    let (part_1, part_2) = solve_day_05(input);

    println!("Lowest reachable location: {}", part_1);

    println!("Lowest reachable location (part 2): {}", part_2);
}

#[test]
fn example() {
    let (part_1, part_2) = solve_day_05("../input1.txt");
    assert_eq!(part_1, 35);
    assert_eq!(part_2, 46);
}
// let (times, distances) = input.split_once("\n").expect("should ne valid split");
// let times = times
//     .strip_prefix("Time:")
//     .expect("should exist")
//     .trim()
//     .split_ascii_whitespace()
//     .map(|time| time.parse::<u32>())
//     .collect::<Result<Vec<u32>, ParseIntError>>();
// let distances = distances
//     .strip_prefix("Distance:")
//     .expect("should exist")
//     .trim()
//     .replace(" ", "")
//     .parse::<u64>()
//     .expect("shoud be a number");
// info!(times);
// info!(distances);
