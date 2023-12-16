use std::{collections::HashMap, error::Error, fs, time::Instant};
use tracing::info;

const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKNOWN: u8 = b'?';

fn arrangements(data: &[u8], groups: &[u32]) -> u64 {
    let mut cache = HashMap::default();
    dfs(&mut cache, data, groups, 0, 0, 0)
}

#[tracing::instrument]
fn dfs(
    cache: &mut HashMap<(usize, usize, u32), u64>,
    data: &[u8],
    groups: &[u32],
    from: usize,
    group: usize,
    size: u32,
) -> u64 {
    if from >= data.len() {
        // exhausted all groups
        if group >= groups.len() {
            return 1;
        }

        // the line ends with a "damaged" symbol and we've matched that last group
        if group == groups.len() - 1 && groups[group] == size {
            return 1;
        }

        return 0;
    }

    match data[from] {
        OPERATIONAL => {
            // skip sequence of operational spots
            if size == 0 {
                return dfs(cache, data, groups, from + 1, group, size);
            }

            // the current combination failed to match a proper sequence from the input
            if group >= groups.len() || size != groups[group] {
                return 0;
            }

            // we have a match: process the next group
            return dfs(cache, data, groups, from + 1, group + 1, 0);
        }

        DAMAGED => {
            // we do not expect more damaged spots, thus failed to match
            if group >= groups.len() || size + 1 > groups[group] {
                return 0;
            }

            return dfs(cache, data, groups, from + 1, group, size + 1);
        }

        UNKNOWN => {
            if let Some(answer) = cache.get(&(from, group, size)).copied() {
                return answer;
            }

            let mut ways = 0;

            // if we did not encounter any damaged cells,
            // we can treat this one as undamaged
            if size == 0 {
                ways += dfs(cache, data, groups, from + 1, group, size);
            }

            // if we need more damaged cells to complete our match,
            // we can treat the current cell as damaged
            if group < groups.len() && size < groups[group] {
                ways += dfs(cache, data, groups, from + 1, group, size + 1);
            }

            // we have the correct number of damaged cells, so we can just
            // treat this one as undamaged in order to complete the match
            if group < groups.len() && size == groups[group] {
                ways += dfs(cache, data, groups, from + 1, group + 1, 0);
            }

            cache.insert((from, group, size), ways);
            return ways;
        }

        _ => unreachable!(),
    };
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let result = input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l.trim(),
                r.trim()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(data, groups)| arrangements(data.as_bytes(), &groups))
        .sum::<u64>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let input = fs::read_to_string("test_input.txt").expect("should be string");
        assert_eq!("21", process(input.as_str())?);
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
