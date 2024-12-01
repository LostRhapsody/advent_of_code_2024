// Answer: 2285373

use std::fs::File;
use std::io::{prelude::*, BufReader};
use anyhow::Result;

const INPUT_PATH: &str = "src/inputs/day_one/puzzle_input.txt";
const RESULTS_PATH: &str = "src/outputs/day_one/puzzle_output.txt";

pub fn main() -> Result<()>{
  println!("Reading in file: {} ", INPUT_PATH);

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

  let mut final_answer: u64 = 0;
  results.iter().for_each(|answer|{
    final_answer += *answer;
  });

  let mut output = File::create(RESULTS_PATH)?;
  match write!(output, "{}", final_answer.to_string()) {
    Ok(_) => println!("Saved output to file"),
    Err(err) => println!("Error writing answer to file: {}", err),
  }

  Ok(())
}