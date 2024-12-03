use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT_PATH: &str = "src/inputs/day_one/puzzle_input.txt";
const RESULTS_PATH: &str = "src/outputs/day_one/puzzle_output.txt";

pub fn solve() -> Result<()> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);

    // Pre-allocate vectors with capacity to avoid reallocations
    let (mut left_column, mut right_column) =
        reader
            .lines()
            .try_fold((Vec::new(), Vec::new()), |mut acc, line| -> Result<_> {
                let line = line?;
                let mut parts = line.split("   ");
                // Using if let to avoid unwrap and get better error handling
                if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
                    acc.0.push(left.parse::<u64>()?);
                    acc.1.push(right.parse::<u64>()?);
                }
                Ok(acc)
            })?;

    // Part 1: Calculate total distance
    left_column.sort_unstable(); // sort_unstable is faster than stable sort
    right_column.sort_unstable();

    let total_distance: u64 = left_column
        .iter()
        .zip(right_column.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum();

    // Part 2: Calculate similarity score
    // Use HashMap for O(1) lookups instead of nested iteration
    let frequency_map: HashMap<u64, u64> =
        right_column.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_default() += 1;
            map
        });

    let similarity_score: u64 = left_column
        .iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum();

    // Write results
    std::fs::write(
        RESULTS_PATH,
        format!("part 1:{}\npart 2:{}", total_distance, similarity_score),
    )?;

    Ok(())
}
