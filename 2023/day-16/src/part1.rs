use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs,
    time::Instant,
};
use tracing::info;

fn find_energized_quant(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut cache: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut energized: HashSet<(i32, i32)> = HashSet::new();
    let mut light_startpoints: VecDeque<(i32, i32, i32, i32)> = VecDeque::from([(0, -1, 0, 1)]);

    while !light_startpoints.is_empty() {
        let (r, c, dr, dc) = light_startpoints.pop_back().unwrap();
        // info!("{},{}", r, c);
        // info!("q len - {}", light_startpoints.len());
        let r = r + dr;
        let c = c + dc;
        // info!("{},{}", c, r);
        if r < 0 || r as usize >= grid.len() || c < 0 || c as usize >= grid[0].len() {
            continue;
        }
        let ch = grid[r as usize][c as usize];
        // info!("The character is {}", ch);

        // info!("starting check");

        if ch == '.' || (ch == '-' && dc != 0) || (ch == '|' && dr != 0) {
            // info!("scenario 1");
            if !cache.contains(&(r, c, dr, dc)) {
                // info!("{},{},{},{}", r, c, dr, dc);
                cache.insert((r, c, dr, dc));
                light_startpoints.push_front((r, c, dr, dc));
                if !energized.contains(&(r, c)) {
                    energized.insert((r, c));
                }
            }
        } else if ch == '/' {
            // info!("scenario 2");
            let (dr, dc) = (-dc, -dr);
            if !cache.contains(&(r, c, dr, dc)) {
                // info!("{},{},{},{}", r, c, dr, dc);
                cache.insert((r, c, dr, dc));
                light_startpoints.push_front((r, c, dr, dc));
                if !energized.contains(&(r, c)) {
                    energized.insert((r, c));
                }
            }
        } else if ch == '\\' {
            // info!("scenario 3");
            let (dr, dc) = (dc, dr);
            if !cache.contains(&(r, c, dr, dc)) {
                // info!("{},{},{},{}", r, c, dr, dc);
                cache.insert((r, c, dr, dc));
                light_startpoints.push_front((r, c, dr, dc));
                if !energized.contains(&(r, c)) {
                    energized.insert((r, c));
                }
            }
        } else {
            if ch == '|' && dc != 0 {
                // info!("scenario 4");
                for (dr, dc) in [(1, 0), (-1, 0)] {
                    // info!("dr, dc = {},{}", dr, dc);
                    if !cache.contains(&(r, c, dr, dc)) {
                        // info!("{},{},{},{}", r, c, dr, dc);
                        cache.insert((r, c, dr, dc));
                        light_startpoints.push_front((r, c, dr, dc));
                        if !energized.contains(&(r, c)) {
                            energized.insert((r, c));
                        }
                    }
                }
            }
            if ch == '-' && dr != 0 {
                // info!("scenario 5");
                for (dr, dc) in [(0, 1), (0, -1)] {
                    // info!("dr, dc = {},{}", dr, dc);
                    if !cache.contains(&(r, c, dr, dc)) {
                        // info!("{},{},{},{}", r, c, dr, dc);
                        cache.insert((r, c, dr, dc));
                        light_startpoints.push_front((r, c, dr, dc));
                        if !energized.contains(&(r, c)) {
                            energized.insert((r, c));
                        }
                    }
                }
            }
        }
        // info!("end of iteration")
    }
    energized.len() as u32
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let result = find_energized_quant(input);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("46", process(input.as_str())?);
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
