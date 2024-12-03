use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT_PATH: &str = "src/inputs/day_two/puzzle_input.txt";
const RESULTS_PATH: &str = "src/outputs/day_two/puzzle_output.txt";

pub fn solve() -> Result<()> {

  let file = File::open(INPUT_PATH)?;
  let reader = BufReader::new(file);
  let mut safe_reports:usize = 0;
  let mut line_number = 0;

  reader.lines().for_each(|line| {
    let line = line.unwrap();
    line_number += 1;
    let entries: Vec<usize> =  line.split(" ").map(|e| e.parse::<usize>().unwrap()).collect();

    let mut is_ascending = false;
    let mut is_descending = false;
    let mut last_entry: usize = 0;
    let mut is_first_loop = true;
    let mut safe = true;

    for entry in entries {

      // set the initial last entry
      if is_first_loop{
        last_entry = entry;
        is_first_loop = false;
        continue;
      }

      // check if we're ascending or descending, but only if neither are true yet
      if !is_descending && !is_ascending {
        if last_entry > entry {is_descending = true} else {is_ascending = true}
      }

      if last_entry == entry {
        println!("{} not safe, neither increase nor decrease", line_number);
        safe = false;
        break;
      }

      if is_descending && last_entry < entry {
        println!("{} not safe, began descending but is no longer", line_number);
        safe = false;
        break;
      }

      if is_ascending && last_entry > entry {
        println!("{} not safe, began ascending but is no longer", line_number);
        safe = false;
        break;
      }

      // if we're descending (meaning the last entry is greater than current)
      // subtract last entry from current
      if is_descending {
        if (last_entry - entry) > 3 || (last_entry - entry) < 1 {
          println!("{} not safe, larger difference than 1-3", line_number);
        safe = false;
        break;
        }
      }

      // if we're ascending (meaning the current entry is greater than the last)
      // subtract entry from last entry
      if is_ascending {
        if (entry - last_entry) > 3 || (entry - last_entry) < 1 {
          println!("{} not safe, larger difference than 1-3", line_number);
        safe = false;
        break;
        }
      }

      // set the next entry
      if !is_first_loop { last_entry = entry }

    }
    println!("{} safe", line_number);
    if safe {safe_reports += 1}
  });

  // Write results
  std::fs::write(
    RESULTS_PATH,
    format!("part 1:{}\npart 2:", safe_reports)
  )?;

  Ok(())
}