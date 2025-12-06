use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file
///
/// this function take a file path as a parameter
/// and returns a list with all the lines in the file.
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`io::Result<Vec<String>>`] with all the lines in the file. For each line the words are separated
///
/// # Examples
///
/// ```
/// let input = read_file_to_list("in.txt")?;
/// ```
fn read_file_to_list(path: String) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => lines.push(content),
            Err(e) => eprintln!("Error reading a line: {}", e),
        }
    }

    Ok(lines)
}

/// reads vertical number from a matrix. It ignores the last row of the matrix
///
/// this function take a matrix as a parameter and return the number on a column of the matrix
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`io::Result<i64>`] with the number or error if the column doesnt contain a number
/// # Examples
///
/// ```
/// let number = get_column_number([1],0)?;
/// ```
fn get_column_number(input: &Vec<String>, column_index: usize) -> io::Result<i64> {
    let mut number = String::new();

    for line in input {
        if let Some(ch) = line.chars().nth(column_index) {
            if ch.is_ascii_digit() {
                number.push(ch);
            }
        }
    }

    match number.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid number/empty column",
        )),
    }
}

/// calculator part two
///
/// for each column in the input elaborate each number based on the operation present in the last line
///
/// # Arguments
///
/// * `lines` - lines from the input
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
///
fn column_calculator(lines: Vec<String>) -> i64 {
    if lines.is_empty() {
        return 0;
    }

    let mut ris: i64 = 0;
    let mut numbers: Vec<i64> = Vec::new();
    let mut operation_numbers: Vec<Vec<i64>> = Vec::new(); // numbers related to an operation
    let operators = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    let mut operation_number = 0;

    for column in 0..lines[0].len() {
        match get_column_number(&lines, column) {
            Ok(n) => {
                numbers.push(n);
            }
            Err(_) => {
                // empty line means we need to calculate the operation related to the "numbers" vector
                // we save the numbers on the operation_numbers vector and will the the calculations after we finished parsing the matrix
                operation_numbers.push(numbers.clone());

                numbers.clear();
                operation_number += 1;
            }
        }
    }
    operation_numbers.push(numbers.clone());    //at the end of the file there isnt a empty column

    for operation_index in 0..operators.len() {
        match operators[operation_index] {
            "+" => {
                ris += operation_numbers[operation_index].iter().sum::<i64>();
            }
            "*" => {
                ris += operation_numbers[operation_index].iter().product::<i64>();
            }
            _ => panic!("Invalid operation: {}", operation_number),
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
