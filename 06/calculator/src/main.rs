use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file
///
/// this function take a file path as a parameter
/// and returns a list with all the lines in the file.
/// [`Vec<Vec<String>>`] the first vector is for lines. 
/// The second is for the words in each line
/// 
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`Vec<Vec<String>>`] with all the lines in the file. For each line the words are separated
///
/// # Examples
///
/// ```
/// let input = read_file_to_list("in.txt")?;
/// ```
fn read_file_to_list(path: String) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => lines.push(content.split_ascii_whitespace().map(|s| s.to_string()).collect()),
            Err(e) => eprintln!("Error reading a line: {}", e),
        }
    }

    Ok(lines)
}

/// calculator
///
/// for each column in the input elaborate each number based on the operation present in the last line
///
/// # Arguments
///
/// * `input` - lines from the input
///
/// # Returns
///
/// A [`i64`] sum of the results from the calculations
///
/// # Examples
///
/// ```
/// let res = column_calculator(read_file_to_list("in.txt"));
/// ```
fn column_calculator(input: Vec<Vec<String>>) -> i64 {
    if input.is_empty() {
        return 0;
    }

    let mut ris: i64 = 0;

    for column in 0..input[0].len() {
        let operation = input[input.len()-1][column].clone(); //last row
        match operation.as_str() {
            "+" => {
                let mut sum = 0;
                for i in 0..input.len()-1 {
                    sum += input[i][column].parse::<i64>().unwrap();
                }
                ris += sum;
            },
            "*" => {
                let mut product = 1;
                for i in 0..input.len()-1 {
                    product *= input[i][column].parse::<i64>().unwrap();
                }
                ris += product;
            },
            _ => panic!("Invalid operation: {}", operation),
        }
    }

    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        column_calculator(read_file_to_list(input_file.to_string())?)
    );

    Ok(())
}