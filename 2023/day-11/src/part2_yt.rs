use glam::I64Vec2;
use itertools::Itertools;
use std::{error::Error, fs, time::Instant};
use tracing::{info, span, Level};

#[tracing::instrument(skip(input))]
pub fn process(input: &str, expansion_size: i64) -> Result<String, Box<dyn Error>> {
    let empty_rows = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| line.chars().all(|c| c == '.').then_some(index))
        .collect::<Vec<usize>>();

    let mut columns = input.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let empty_columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .enumerate()
    .filter_map(|(index, column)| column.iter().all(|c| c == &'.').then_some(index))
    .collect::<Vec<usize>>();

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(I64Vec2::new(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<I64Vec2>>();
    let count = galaxies
        .iter()
        .combinations(2)
        .map(|s| {
            let galaxy_a_id = galaxies.iter().position(|v| v == s[0]).unwrap() + 1;
            let galaxy_b_id = galaxies.iter().position(|v| v == s[1]).unwrap() + 1;
            let my_span = span!(
                Level::INFO,
                "galaxy_map_span",
                ids=format!("{}-{}", galaxy_a_id.min(galaxy_b_id), galaxy_a_id.max(galaxy_b_id)),
                galaxy_a_id,
                galaxy_b_id,
                galaxy_a = ?galaxies
                    .iter()
                    .find(|v| v == &s[0])
                    .unwrap(),
                galaxy_b = ?galaxies
                    .iter()
                    .find(|v| v == &s[1])
                    .unwrap() // duplicates = acc.get(&index)
            );
            my_span.in_scope(|| {
                let galaxy_a_expanded = {
                    let expand_steps_row = empty_rows
                        .iter()
                        .position(|row| row > &(s[0].y as usize))
                        .unwrap_or(empty_rows.len());
                    let expand_steps_columns = empty_columns
                        .iter()
                        .position(|column| column > &(s[0].x as usize))
                        .unwrap_or(empty_columns.len());

                    *s[0]
                        + I64Vec2::new(
                            expand_steps_columns as i64 * (expansion_size - 1),
                            expand_steps_row as i64 * (expansion_size - 1),
                        )
                };

                let galaxy_b_expanded = {
                    let expand_steps_row = empty_rows
                        .iter()
                        .position(|row| row > &(s[1].y as usize))
                        .unwrap_or(empty_rows.len());
                    let expand_steps_columns = empty_columns
                        .iter()
                        .position(|column| column > &(s[1].x as usize))
                        .unwrap_or(empty_columns.len());

                    *s[1]
                        + I64Vec2::new(
                            expand_steps_columns as i64 * (expansion_size - 1),
                            expand_steps_row as i64 * (expansion_size - 1),
                        )
                };

                let v = (galaxy_a_expanded - galaxy_b_expanded).abs();
                let distance = (v.x + v.y).abs();
                distance
            })
        })
        .sum::<i64>();
    Ok(count.to_string())
}

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("should be string");
    println!(
        "{:?}",
        process(input.as_str(), 1000000).expect("should be a string")
    );
    let elapsed = now.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
