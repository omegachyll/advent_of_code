use itertools::Itertools;
use std::str;
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let mut final_ch: &str = &"sdff";
    let mut index: u32 = 0;
    for (i, ch) in input.as_bytes().windows(4).skip(1).enumerate() {
        let unique_count = ch.iter().unique().count();
        if unique_count == 4 {
            index = (i + 5) as u32;
            final_ch = str::from_utf8(ch)?;
            break;
        }
    }
    info!(index);
    info!("{}", final_ch);
    let result = index;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "5")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "6")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11")]
    fn test_process_line(#[case] line: &str, #[case] expected: &str) -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(expected, process(line)?);
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
