//note: i think a linear solution ( O(N) where N is the number of banks) is possible

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
/// finds the max number obtainable selection two digits from each value of the input vector
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
        let mut first_bat_bank_pos = 0; //id = bank position 
        let mut second_bat_bank_pos = 1;
        for new_bank_bat_pos in 2..bank.len() {
            // i do the comparison between chars because its the same as comparing the numbers
            let first_bat_bank_value = bank.chars().nth(first_bat_bank_pos).unwrap();
            let second_bat_bank_value = bank.chars().nth(second_bat_bank_pos).unwrap();
            let new_bank_bat_pos_value = bank.chars().nth(new_bank_bat_pos).unwrap();
            if first_bat_bank_value < second_bat_bank_value {
                first_bat_bank_pos = second_bat_bank_pos;
                second_bat_bank_pos = new_bank_bat_pos;
            } else if new_bank_bat_pos_value > second_bat_bank_value {
                second_bat_bank_pos = new_bank_bat_pos;
            }
        }

        let resulting_joltage: u64 = format!(
            "{}{}",
            bank.chars().nth(first_bat_bank_pos).unwrap(),
            bank.chars().nth(second_bat_bank_pos).unwrap()
        )
        .parse()
        .unwrap();

        //debug
        //print!("bank: {}, 1:{}, 2:{}\n",bank,first_bat_bank_pos,second_bat_bank_pos);

        ris += resulting_joltage;
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
