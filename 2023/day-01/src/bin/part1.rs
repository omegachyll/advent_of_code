use std::error::Error;
use std::fs;
use std::result::Result;

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));

            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("should be string");
    // println!("{}", input.as_str())
    println!("{:?}", process(input.as_str()).expect("should be a string"))
}
