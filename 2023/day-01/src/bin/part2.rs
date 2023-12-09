use std::error::Error;
use std::fs;
use std::result::Result;

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let output = input.lines().map(process_line).sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    // println!("{}", line);
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", "29")]
    #[case("eightwothree", "83")]
    #[case("abcone2threexyz", "13")]
    #[case("xtwone3four", "24")]
    #[case("4nineeightseven2", "42")]
    #[case("zoneight234", "14")]
    #[case("7pqrstsixteen", "76")]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input2.txt").expect("should be string");
    println!("{:?}", process(input.as_str()).expect("should be a string"))
}
