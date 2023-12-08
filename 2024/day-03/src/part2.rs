use itertools::Itertools;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::result::Result;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y as i32, x as i32),
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("should be a number"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("Shouldn't happen"),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }

    let mut total = 0;
    for symbol in map
        .iter()
        .filter(|(_, value)| matches!(value, Value::Symbol('*')))
    {
        println!("symbol {:?}", symbol);
        //(x,y)
        let positions = [
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| {
                // outer_pos.x + pos.x, .y + .y
                (
                    outer_pos.0 + symbol.0 .1 as i32,
                    outer_pos.1 + symbol.0 .0 as i32,
                )
            })
            .collect();
        // .filter(|num| num_list.);
        // dbg!(pos_to_check.len(), pos_to_check);

        let mut indexes_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list
                    .iter()
                    .find(|(num_pos, _)| num_pos == &pos)
                    .is_some()
                {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            println!(" ");
            total += indexes_of_numbers
                .iter()
                .unique()
                .inspect(|_| println!(" "))
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .inspect(|(_, num)| println!("{num}"))
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }

    // sum part numb
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = fs::read_to_string("input_test.txt").expect("Should be a string");
        assert_eq!(
            "467835",
            process(input.as_str()).expect("should be a string")
        )
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be a string");
    println!("{}", process(&input).expect("should be a string"))
}
