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
/// let in = read_file_to_list("in.txt");
/// ```
fn read_file_to_list(path: String) -> io::Result<Vec<String>> {
    let file: File = File::open(path)?;
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

/// find splits number
///
/// this function solves the problem at https://adventofcode.com/2025/day/7
///
/// # Arguments
///
/// * `input` - a vector that contains in every cell a line from the input
///
/// # Returns
///
/// A [`u64`] the solution to the problem(number of splits)
///
/// # Examples
///
/// ```
/// let res = find_max_joltage(read_file_to_list("in.txt"));
/// ```
fn count_splits(input: Vec<String>) -> u64 {
    let mut ris = 0;

    let mut beams_positions: Vec<char> = Vec::new();
    for line_index in 0..input.len() {
        let line = &input[line_index];
        for i in 0..line.len() {
            //analysing the i line from the input and updating the beams_positions
            if line_index == 0 {
                //first line -> push into beams_positions
                if line.chars().nth(i).unwrap() == 'S' {
                    //tachyon beam enters the manifold
                    beams_positions.push('|'); //| = beam symbol
                } else {
                    beams_positions.push('.'); //. = empty space symbol
                }
            } else {
                if line.chars().nth(i).unwrap() == 'S' {
                    //tachyon beam enters the manifold
                    beams_positions[i] = '|'; //| = beam symbol
                } else if line.chars().nth(i).unwrap() == '^' && beams_positions[i] == '|' {
                    ris+=1;
                    beams_positions[i] = '.';
                    beams_positions[i - 1] = '|';
                    beams_positions[i + 1] = '|';
                }
            }
        }
        //println!("{}", beams_positions.iter().collect::<String>());
    }

    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        count_splits(read_file_to_list(input_file.to_string())?)
    );

    Ok(())
}
