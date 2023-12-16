use itertools::Itertools;
use std::str;
use std::{error::Error, fs, time::Instant};
use tracing::info;

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let win = 14 as usize;
    // let mut final_ch: &str = &"";
    let mut index: u32 = 0;
    for (i, ch) in input.as_bytes().windows(win).skip(1).enumerate() {
        let unique_count = ch.iter().unique().count();
        if unique_count == win {
            index = (i + win + 1) as u32;
            // final_ch = str::from_utf8(ch)?;
            break;
        }
    }
    // info!(index);
    // info!("{}", final_ch);
    let result = index;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "23")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "23")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26")]
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
