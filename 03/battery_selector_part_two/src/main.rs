//note: i think a linear solution ( O(N)) is possible

use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file line by line
///
/// this function take a file path as a parameter
/// and returns a list with all the lines in the file
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`Vec<String>`] with all the lines of the file.
///
/// # Examples
///
/// ```
/// let s = read_file_to_list("in.txt");
/// ```
fn read_file_to_list(path: String) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => lines.push(content),
            Err(e) => eprintln!("Error reading the line: {}", e),
        }
    }

    Ok(lines)
}

/// find_max_joltage
///
/// finds the max number obtainable concatenating 12 digits for each of the input vector values and sum all the max numbers
///
/// # Arguments
///
/// * `banks` - a vector that has a number in every cell rappresenting a set of batteries
///
/// # Returns
///
/// A [`u64`] sum of the max number obtainable for every cell of the vector
///
/// # Examples
///
/// ```
/// let res = find_max_joltage(read_file_to_list("in.txt"));
/// ```
fn find_max_joltage(banks: Vec<String>) -> u64 {
    let mut ris: u64 = 0;

    for bank in banks {
        let mut bank_choosen_batteries: Vec<usize> = Vec::new(); //array of indexes
        let bank_chars: Vec<char> = bank.chars().collect(); // bank.chars().nth is O(N), so its convenient to just run it once
        for i in 0..12 {
            //initialization of bank_choosen_batteries with the first 12 batteries
            bank_choosen_batteries.push(i);
        }

        for new_considered_bat in 12..bank.len() {
            //shifting the values if within the already selected batteries the next of each one is greater than the next
            //i need to allow the new considered battery in the current bat. selection if either is convient to shift the currently selected batteries because of their current values or either the new battery is greater than the last one
            let mut shift_happened = false;
            for i in 0..11 {
                // analysing the bank_choosen_batteries to check if a shift is convenient
                let current_batt_value = bank_chars[bank_choosen_batteries[i]];
                let next_batt_value = bank_chars[bank_choosen_batteries[i + 1]];
                if next_batt_value > current_batt_value {
                    // shifting all the values starting from i in bank_choosen_batteries e inserting the new battery (new_considered_bat) in the last slot
                    bank_choosen_batteries.remove(i);
                    bank_choosen_batteries.push(new_considered_bat);
                    shift_happened = true;
                    break;
                }
            }
            if !shift_happened {
                let new_considered_bat_value = bank_chars[new_considered_bat];
                let last_bank_choosen_batteries_bat_value = bank_chars[bank_choosen_batteries[11]];
                if new_considered_bat_value > last_bank_choosen_batteries_bat_value {
                    bank_choosen_batteries[11] = new_considered_bat;
                }
            }
        }

        let mut resulting_joltage: String = String::from("");
        for index in bank_choosen_batteries {
            resulting_joltage.push(bank_chars[index]);
        }

        //debug
        //print!("bank:{}, selected:{}\n",bank, resulting_joltage.parse::<u64>().unwrap());

        ris += resulting_joltage.parse::<u64>().unwrap();
    }

    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        find_max_joltage(read_file_to_list(input_file.to_string())?)
    );

    Ok(())
}
