use std::fs::File;
use std::io::{self, BufRead, BufReader};

//input file reading line by line and returns a list with the lines of the file
fn read_file_to_list() -> io::Result<Vec<String>> {
    let file = File::open("in.txt")?;
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

// solving the problem by simulating the safe dial movement
fn count_dial_when_zero(dial_movements: Vec<String>) -> i32 {
    let starting_pos = 50;
    let mut current_pos = starting_pos;
    let mut ris: i32 = 0;

    for movement in dial_movements {
        // do movement
        let mut shift_str = &movement[1..];
        let mut shift_int: i32 = match shift_str.parse::<i32>() {
            Ok(num) => num, // Parse successfully
            Err(_) => 0,    // If parsing fails, assume 0
        };

        //dial movement simulation click by click
        while shift_int != 0 {
            if movement.starts_with("L") {
                //left
                current_pos -= 1;
                shift_int -= 1;
            } else if movement.starts_with("R") {
                current_pos += 1;
                shift_int -= 1;
            }
            if current_pos == 100 {
                current_pos = 0;
            } else if current_pos == -1 {
                current_pos = 99;
            }
        }

        if current_pos == 0 {
            ris += 1;
        }

        println!("current pos:{}, ris:{},", current_pos, ris);
    }
    return ris;
}

fn main() {
    let file = match read_file_to_list() {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        }
    };
    print!("{}", count_dial_when_zero(file));
}
