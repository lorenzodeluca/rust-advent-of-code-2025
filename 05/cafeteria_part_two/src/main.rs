use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// reads a file line by line
///
/// this function take a file path as a parameter
/// and returns a list with all the lines in the file.
/// This function could be improved by optimizing the ranges rappresenting the fresh ingredients to remove duplicates
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// [`Result<(Vec<(i64, i64)>,Vec<i64>)>`] -> Vec<(i64, i64)> are the ranges of fresh food.Vec<i64> are the id that needs to be analysed
///
/// # Examples
///
/// ```
/// let input = read_file("in.txt");
/// ```
fn read_file(path: String) -> Result<(Vec<(i64, i64)>, Vec<i64>), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids_to_check: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let content = line?;

        if content.contains('-') {
            let (start, end) = content.split_once('-').ok_or("format error")?;

            ranges.push((start.trim().parse::<i64>()?, end.trim().parse::<i64>()?));
        } else if !content.trim().is_empty() {
            ids_to_check.push(content.trim().parse::<i64>()?);
        }
    }

    Ok((ranges, ids_to_check))
}

/// solution for the day 5.2 advent of code 2025 challenge
///
/// given the ranges, count how many ids are inside the ranges
///
/// # Arguments
///
/// * `fresh_ranges` - a list of ranges
///
/// # Returns
///
/// [`i64`] how many ids fell in the ranges
///
/// # Examples
///
/// ```
/// let res = count_fresh_ids(get_ranges_from_string(read_file("in.txt")));
/// ```
fn count_fresh_ids(mut fresh_ranges: Vec<(i64, i64)>) -> i64 {
    if fresh_ranges.is_empty() {
        return 0;
    }

    let mut ris = 0;
    let mut last_counted_number;

    fresh_ranges.sort_by_key(|(a, _)| *a);
    last_counted_number = fresh_ranges[0].0 - 1;

    for (start, end) in fresh_ranges {
        let mut count_from = start;
        let count_to = end;

        if last_counted_number >= count_from {
            count_from = last_counted_number + 1;
        }
        if count_from > count_to {
            continue;
        }
        ris += count_to - count_from + 1;
        last_counted_number = count_to;
    }
    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    let (ranges, _) = read_file(input_file.to_string())?;

    print!("{}", count_fresh_ids(ranges));

    Ok(())
}
