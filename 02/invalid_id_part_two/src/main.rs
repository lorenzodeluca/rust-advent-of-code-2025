use std::arch::x86_64::_SIDD_NEGATIVE_POLARITY;
use std::fs::File;
use std::io::{self, Read};

/// reads a file
///
/// this function take a file path as a parameter
/// and returns the whole file content
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`String`] with the content of the file.
///
/// # Examples
///
/// ```
/// let s = read_file("in.txt");
/// ```

fn read_file(path: String) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
            return Err(err);
        }
    }

    Ok(content)
}

/// read the ranges from the file
///
/// read the ranges from the file and returns a list with the ranges
///
/// # Arguments
///
/// * `string_ranges` - a list of ranges comma separated. (ex 516015-668918,222165343-222281089)
///
/// # Returns
///
/// A [`Vec<(u64, u64)>`] with in every cell a couple with the strarting range and the ending range
///
/// # Examples
///
/// ```
/// let s = get_ranges_from_string(read_file("in.txt"));
/// ```
fn get_ranges_from_string(
    string_ranges: String,
) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    for part in string_ranges.split(',') {
        let (start, end) = part.split_once('-').ok_or("single range format error")?;
        let start: u64 = start.trim().parse()?;
        let end: u64 = end.trim().parse()?;
        res.push((start, end));
    }
    Ok(res)
}

/// solution for the day 2 advent of code 2025 challenge
///
/// given the ranges, detect the "wrong" ids in which the first part of the number is the same as the second
///
/// # Arguments
///
/// * `ranges` - a list of ranges
///
/// # Returns
///
/// [`u64`] the sum of the wrong ids in the given ranges
///
/// # Examples
///
/// ```
/// let s = sum_wrong_ids(get_ranges_from_string(read_file("in.txt")));
/// ```
fn sum_wrong_ids(ranges: Vec<(u64, u64)>) -> u64 {
    let mut ris: u64 = 0;

    for (start, end) in ranges {
        for id in start..=end {
            let id_string: String = id.to_string();
            let len = id_string.len();

            for sub_len in 1..=(len / 2) {

                if len % sub_len != 0 {
                    continue;
                }

                let mut is_repeated = true; //is_repeated means the string should be counted(and added to the sum) because it match the problem criteria for "wrong" ids
                for substring_id in 1..(len / sub_len) { //substring_id equals 1 means first substring, 
                    let substring_id_starting_index = substring_id * sub_len;
                    if id_string[0..sub_len] != id_string[substring_id_starting_index..substring_id_starting_index + sub_len] {
                        is_repeated = false;
                        break;
                    }
                }
                if is_repeated {
                    // println!("debug -> matched {} with length {}", id, sub_len);
                    ris += id;
                    break;
                }
            }
        }
    }

    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        sum_wrong_ids(get_ranges_from_string(read_file(input_file.to_string())?)?)
    );

    Ok(())
}
