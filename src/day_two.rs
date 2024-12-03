use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT_PATH: &str = "src/inputs/day_two/puzzle_input.txt";
const RESULTS_PATH: &str = "src/outputs/day_two/puzzle_output.txt";

pub fn solve_v2() -> Result<()> {
    /*
    second attempt
    my goal:
    iterate through each report. Find the total number of problems in the report.
    Then, iterate through it again, removing a single level, checking to see if there are
    less than 2 problems without that level. If yes, mark it as safe and move onto the next report.
    */
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut safe_reports: usize = 0;
    let mut line_number = 0;

    for line in reader.lines() {
        // get report
        let line = line.unwrap();
        line_number += 1;

        // get vector of entries from report
        let report: Vec<usize> = line
            .split(" ")
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        // number of problems found in report
        let mut problem_counter = 0;

        let mut last_entry = None;

        // iterate through report
        report.iter().for_each(|entry| {
            // if last_entry has been set, compare it to current entry
            if last_entry.is_some() {
                // check for problems here
            }
            // set the last entry at end of loop
            last_entry = Some(entry);
        });

        // if we only encountered 1 (or 0) unsafe levels, report is safe, move on to next report.
        if problem_counter < 2 {
            safe_reports += 1;
            continue;
        }

        /*
        if we had 2 problems iterate again, removing one
        entry at a time until there is only 1 problem remaining
        */
        if problem_counter == 2 {
            let report_length = report.len();
            // loop through the NUMBER of entries, not the entry itself
            for index in 0..report_length {
                // clone reports into altered reports
                let mut altered_report = report.clone();
                // remove the current index
                altered_report.remove(index);
                // iterate through report again and perform checks without the current index
                let mut last_entry = None;
                let mut problem_counter = 0;
                altered_report.iter().for_each(|entry| {
                    // if last_entry has been set, compare it to current entry
                    if last_entry.is_some() {
                        // check for problems here
                        // if 1 or 0 problems found, mark as safe
                    }
                    // set the last entry at end of loop
                    last_entry = Some(entry);
                });

                /*
                if only 1 or 0 problems found, break current loop
                (which is the loop where we remove 1 entry from report and double check it)
                and mark report as safe
                */
                if problem_counter < 2 {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    // Write results
    println!("Safe reports: {}", safe_reports);
    std::fs::write(RESULTS_PATH, format!("part 1:{}\npart 2:", safe_reports))?;

    Ok(())
}

pub fn solve() -> Result<()> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut safe_reports: usize = 0;
    let mut line_number = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        line_number += 1;
        let entries: Vec<usize> = line
            .split(" ")
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        let mut is_ascending = false;
        let mut is_descending = false;
        let mut last_entry: usize = 0;
        let mut is_first_loop = true;
        let mut safe = true;
        // the number of "problems" we can encounter but still mark the report as safe
        let mut problem_allocation = 1;
        // Our first loop will check how many "levels" are unsafe per report, 1 or more.
        // if it's only one unsafe level, we remove that level then add the adjused report to this new vec
        let mut adjusted_entries: Vec<usize> = Vec::new();

        for entry in entries {
            // setup vvv
            // set the initial last entry
            if is_first_loop {
                last_entry = entry;
                is_first_loop = false;
                continue;
            }

            // check if we're ascending or descending, but only if neither are true yet
            if !is_descending && !is_ascending {
                if last_entry > entry {
                    is_descending = true;
                } else {
                    is_ascending = true;
                }
            }
            // setup ^^^

            // if they are duplicates
            if last_entry == entry {
                // if there are safe problems allocated, reduce by 1
                if problem_allocation > 0 {
                    problem_allocation -= 1;
                    // skip to the next loop iter in case this level fails on something else
                    last_entry = entry;
                    continue;
                }
            }

            // if we started asc instead of desc whilst in desc mode
            if is_descending && last_entry < entry {
                // if there are safe problems allocated, reduce by 1
                if problem_allocation > 0 {
                    problem_allocation -= 1;
                    // skip to the next loop iter in case this level fails on something else
                    last_entry = entry;
                    continue;
                }
            }

            // if we started desc instead of asc while in asc mode
            if is_ascending && last_entry > entry {
                // if there are safe problems allocated, reduce by 1
                if problem_allocation > 0 {
                    problem_allocation -= 1;
                    // skip to the next loop iter in case this level fails on something else
                    last_entry = entry;
                    continue;
                }
            }

            // if the difference is greater than 3
            // subtract based on whats larger, not what "mode" we're in (asc/desc)
            // the mode doesn't really matter. All we're doing is checking for a difference of more than 3
            // note: if the "differenc" is less than 1, they are equal, and we already check for that above.
            if last_entry > entry {
                if last_entry - entry > 3 {
                    // if we have allocated problems we can skip this. Otherwise... bad
                    if problem_allocation > 0 {
                        problem_allocation -= 1;
                        // skip to the next loop iter in case this level fails on something else
                        last_entry = entry;
                        continue;
                    }
                }
            } else {
                if entry - last_entry > 3 {
                    // if we have allocated problems we can skip this. Otherwise... bad
                    if problem_allocation > 0 {
                        problem_allocation -= 1;
                        // skip to the next loop iter in case this level fails on something else
                        last_entry = entry;
                        continue;
                    }
                }
            }

            // set the next entry
            if !is_first_loop {
                last_entry = entry
            } else {
                // if we encounter no issues add to the adjusted array
                adjusted_entries.push(last_entry);
                last_entry = entry
            }
        }

        // reset our flags
        is_ascending = false;
        is_descending = false;
        last_entry = 0;
        is_first_loop = true;

        // go through the entries AGAIN. This time, we removed at least 1 level from
        // unsafe reports to try and make them safe. Perform all the same checks.
        // Also, this time we don't need to take our problem_allocation into account.
        for entry in adjusted_entries {
            // setup vvv
            // set the initial last entry
            if is_first_loop {
                last_entry = entry;
                is_first_loop = false;
                continue;
            }

            // check if we're ascending or descending, but only if neither are true yet
            if !is_descending && !is_ascending {
                if last_entry > entry {
                    is_descending = true;
                } else {
                    is_ascending = true;
                }
            }
            // setup ^^^

            // if they are duplicates
            if last_entry == entry {
                println!("{} not safe, duplicate entries", line_number);
                safe = false;
                break;
            }

            // if we started asc instead of desc whilst in desc mode
            if is_descending && last_entry < entry {
                println!("{} not safe, started desc but began asc", line_number);
                safe = false;
                break;
            }

            // if we started desc instead of asc while in asc mode
            if is_ascending && last_entry > entry {
                println!("{} not safe, started asc but began desc", line_number);
                safe = false;
                break;
            }

            // if the difference is greater than 3
            // subtract based on whats larger, not what "mode" we're in (asc/desc)
            // the mode doesn't really matter. All we're doing is checking for a difference of more than 3
            // note: if the "differenc" is less than 1, they are equal, and we already check for that above.
            if last_entry > entry {
                if last_entry - entry > 3 {
                    println!("{} not safe, diff greater than 3", line_number);
                    safe = false;
                    break;
                }
            } else {
                if entry - last_entry > 3 {
                    println!("{} not safe, diff greater than 3", line_number);
                    safe = false;
                    break;
                }
            }

            // set the next entry
            if !is_first_loop {
                last_entry = entry
            }
        }
        println!("{} safe", line_number);
        if safe {
            safe_reports += 1
        }
    });

    // Write results
    println!("Safe reports: {}", safe_reports);
    std::fs::write(RESULTS_PATH, format!("part 1:{}\npart 2:", safe_reports))?;

    Ok(())
}
