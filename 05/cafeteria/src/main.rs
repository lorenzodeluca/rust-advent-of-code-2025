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
/// [`Result<(Vec<(u64, u64)>,Vec<u64>)>`] -> Vec<(u64, u64)> are the ranges of fresh food.Vec<u64> are the id that needs to be analysed
///
/// # Examples
///
/// ```
/// let input = read_file("in.txt");
/// ```
fn read_file(path: String) -> Result<(Vec<(u64, u64)>, Vec<u64>), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids_to_check: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let content = line?;

        if content.contains('-') {
            let (start, end) = content.split_once('-').ok_or("format error")?;

            ranges.push((start.trim().parse::<u64>()?, end.trim().parse::<u64>()?));
        } else if !content.trim().is_empty() {
            ids_to_check.push(content.trim().parse::<u64>()?);
        }
    }

    Ok((ranges, ids_to_check))
}

/// solution for the day 5 advent of code 2025 challenge
///
/// given the ranges, count how many ids are inside the ranges
///
/// # Arguments
///
/// * `fresh_ranges` - a list of ranges
/// * `ids_to_check` - ids that needs to be checked
///
/// # Returns
///
/// [`u64`] how many ids fell in the ranges
///
/// # Examples
///
/// ```
/// let res = count_fresh_ids(get_ranges_from_string(read_file("in.txt")));
/// ```
fn count_fresh_ids(fresh_ranges: Vec<(u64, u64)>, ids_to_check: Vec<u64>) -> u64 {
    return ids_to_check
        .into_iter()
        .filter(|id| fresh_ranges.iter().any(|(a, b)| id >= a && id <= b))
        .count() as u64;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    let (ranges, ids) = read_file(input_file.to_string())?;

    print!("{}", count_fresh_ids(ranges, ids));

    Ok(())
}
