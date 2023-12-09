use std::result::Result;
use std::error::Error;
use std::fs;

fn process(
    input: &str,
) {

    let output = input
        .lines()
        .map(process_line);
}

fn process_line(line: &str) {

    // println!("{}", line);
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        // println!("{}", reduced_line);
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
            let temp = reduced_line.chars().next().unwrap();
            println!("{}", temp);
            temp
        };

        result.to_digit(10)
    });

    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next())

    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
}

#[cfg(test)]
mod tests{
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine","29")]
    #[case("eightwothree","83")]
    #[case("abcone2threexyz","13")]
    #[case("xtwone3four","24")]
    #[case("4nineeightseven2","42")]
    #[case("zoneight234","14")]
    #[case("7pqrstsixteen","76")]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32
    ) {
        process(line);
        assert_eq!(expected, expected)
    }

}

fn main() {
    let input = fs::read_to_string("input2.txt")
        .expect("should be string");
    process_line("7pqrstsixteen")
}