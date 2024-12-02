// Answer: 2285373

use std::fs::File;
use std::io::{prelude::*, BufReader};
use anyhow::Result;
use std::collections::BTreeSet;

const INPUT_PATH: &str = "src/inputs/day_one/puzzle_input.txt";
const RESULTS_PATH: &str = "src/outputs/day_one/puzzle_output.txt";

pub fn solve() -> Result<()>{

  let file = File::open(INPUT_PATH)?;
  let reader = BufReader::new(file);

  let mut left_column: Vec<u64> = Vec::new();
  let mut right_column: Vec<u64> = Vec::new();

  for line in reader.lines(){
    let line = line?;
    let location_ids: Vec<&str> = line.split("   ").collect();
    left_column.push(location_ids[0].parse::<u64>()?);
    right_column.push(location_ids[1].parse::<u64>()?);
  }

  left_column.sort();
  right_column.sort();

  let mut results: Vec<u64> = Vec::new();

  for (index, id) in left_column.iter().enumerate(){
    let rc_id = &right_column[index];
    if id > rc_id { results.push(id-rc_id) } else { results.push(rc_id-id);}
  };

  let mut common_scores: u64 = 0;
  left_column.iter().for_each(|lc|{
    let mut tally: u64 = 0;
    right_column.iter().for_each(|rc|{
      if lc == rc {
        tally += 1;
      }
    });
    if tally > 0 {
      common_scores += lc * tally;
    }
  });

  let mut final_answer: u64 = 0;
  results.iter().for_each(|answer|{
    final_answer += *answer;
  });

  let mut output = File::create(RESULTS_PATH)?;
  let write_result = write!(output, "part 1:{}\npart 2:{}", final_answer.to_string(), common_scores.to_string());
  if write_result.is_err() {
    println!("Error writing answer to file");
  }

  Ok(())
}