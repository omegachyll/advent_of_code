use nom::{
    bytes::complete::is_a,
    character::complete::{alphanumeric0, alphanumeric1},
    sequence::tuple,
    IResult,
};
use std::{collections::HashMap, error::Error, fs, time::Instant};
use tracing::info;

fn calc_hash(input: &str) -> u32 {
    input.as_bytes().iter().fold(0, |acc, ch| {
        // info!(acc);
        // info!(ch);
        let mut value = *ch as u32 + acc;
        // info!(value);
        value *= 17;
        // info!(value);
        value %= 256;
        // info!(value);
        value
    })
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let mut lens_boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for item in input.split(",").into_iter() {
        let parsed_data: IResult<&str, (&str, &str, &str)> =
            tuple((alphanumeric1, is_a("=-"), alphanumeric0))(item);
        let (_, (label, operation, fl)) = parsed_data.unwrap();
        let box_number = calc_hash(label);

        if !lens_boxes.contains_key(&box_number) {
            lens_boxes.insert(box_number, Vec::new());
        }

        if operation == "=" {
            let focal_length = fl.parse::<u32>().unwrap();
            let lens_box = lens_boxes.get_mut(&box_number).unwrap();
            let position = lens_box.iter().position(|(l, _)| l == &label);
            if position.is_some() {
                lens_box[position.unwrap()] = (label, focal_length);
            } else {
                lens_box.push((label, focal_length));
            }
        } else if operation == "-" {
            let lens_box = lens_boxes.get_mut(&box_number).unwrap();
            let position = lens_box.iter().position(|(l, _)| l == &label);
            if position.is_some() {
                lens_box.remove(position.unwrap());
            }
        } else {
            info!(operation);
            unreachable!("operatoin should be = or -");
        }
        // info!(box_number);
    }

    // info!(?lens_boxes);

    let result = lens_boxes
        .iter()
        .map(|(b, contents)| {
            contents
                .iter()
                .enumerate()
                .map(|(index, (_, fl))| (b.clone() + 1) * (index as u32 + 1) * fl)
                .sum::<u32>()
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
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("145", process(input.as_str())?);
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
