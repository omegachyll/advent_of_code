use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0, multispace1},
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    time::Instant,
};
use tracing::info;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct PipeInfo<'a> {
    span: SpanIVec2<'a>,
    pipe_type: PipeType,
}

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, PipeType>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag("|").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Vertical,
            }),
            tag("-").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Horizontal,
            }),
            tag("L").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthEast,
            }),
            tag("J").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthWest,
            }),
            tag("7").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthWest,
            }),
            tag("F").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthEast,
            }),
            tag(".").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Ground,
            }),
            tag("S").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::StartingPosition,
            }),
        )),
        multispace0,
    )))(input)?;

    // dbg!(output);
    Ok((
        input,
        output
            .into_iter()
            .filter_map(|pipe_info| {
                (pipe_info.pipe_type != PipeType::Ground)
                    .then_some((pipe_info.span.extra, pipe_info.pipe_type))
            })
            .collect(),
    ))
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let (_input, grid) = parse_grid(Span::new(input)).expect("should be a valid parse");
    let start_position = grid
        .iter()
        .find_map(|(key, value)| (value == &PipeType::StartingPosition).then_some(key))
        .expect("starting position must exist");
    let north = *start_position + IVec2::new(0, -1);
    let north_positions = grid
        .get(&north)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical | PipeType::SouthEast | PipeType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::South, north));
    let south = *start_position + IVec2::new(0, 1);
    let south_positions = grid
        .get(&south)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest => true,
            _ => false,
        })
        .then_some((Direction::North, south));
    let east = *start_position + IVec2::new(1, 0);
    let east_positions = grid
        .get(&east)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::West, east));
    let west = *start_position + IVec2::new(-1, 1);
    let west_positions = grid
        .get(&west)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal | PipeType::SouthEast | PipeType::NorthEast => true,
            _ => false,
        })
        .then_some((Direction::East, west));

    let mut iters = vec![
        north_positions,
        south_positions,
        east_positions,
        west_positions,
    ]
    .into_iter()
    .flatten()
    .map(|tuple| {
        std::iter::successors(Some(tuple), |(from_direction, current_position)| {
            let pipe_type = grid
                .get(current_position)
                .expect("should not be asking for a grid position that doesnt exist");
            info!(?from_direction, ?current_position, ?pipe_type);
            let direction_to_go = match (from_direction, pipe_type) {
                (Direction::North, PipeType::NorthEast) => Direction::East,
                (Direction::North, PipeType::NorthWest) => Direction::West,
                (Direction::North, PipeType::Vertical) => Direction::South,
                (Direction::South, PipeType::SouthEast) => Direction::East,
                (Direction::South, PipeType::SouthWest) => Direction::West,
                (Direction::South, PipeType::Vertical) => Direction::North,
                (Direction::East, PipeType::NorthEast) => Direction::North,
                (Direction::East, PipeType::SouthEast) => Direction::South,
                (Direction::East, PipeType::Horizontal) => Direction::West,
                (Direction::West, PipeType::NorthWest) => Direction::North,
                (Direction::West, PipeType::SouthWest) => Direction::South,
                (Direction::West, PipeType::Horizontal) => Direction::East,
                value => {
                    unreachable!(
                        "should not land on ground or loop off into nowhere, {:?}",
                        value
                    )
                }
            };
            Some(match direction_to_go {
                Direction::North => (Direction::South, *current_position + IVec2::new(0, -1)),
                Direction::South => (Direction::North, *current_position + IVec2::new(0, 1)),
                Direction::East => (Direction::West, *current_position + IVec2::new(1, 0)),
                Direction::West => (Direction::East, *current_position + IVec2::new(-1, 0)),
            })
        })
    });

    let path_a = iters.next().expect("path a should exist");
    let path_b = iters.next().expect("path b should exist");
    let final_postition = std::iter::zip(path_a, path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("should meet in the middle");

    let result = final_postition + 1;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("test_input1.txt", "4")]
    #[case("test_input2.txt", "8")]
    fn test_process(#[case] file: &str, #[case] output: &str) -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::try_init();
        let input = fs::read_to_string(file).expect("should be string");
        assert_eq!(output, process(input.as_str())?);
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
