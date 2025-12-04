use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file line by line and put the characters in a matrix
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
/// A [`Result<Vec<Vec<char>>>`] with all the lines of the file.
///
/// # Examples
///
/// ```
/// let input = read_file_to_matrix("in.txt");
/// ```
fn read_file_to_matrix(path: String) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => out.push(content.chars().collect()),
            Err(e) => eprintln!("Error reading the file: {}", e),
        }
    }

    Ok(out)
}

/// find_accessible_rolls part two
///
/// finds how many rolls of paper(@ in the input) in the input are surrounded by less than four other rolls in the adjacent positions.
/// if a roll is removable after the analysis we will remove it and after we will repeat everything on the new grid
/// # Arguments
///
/// * `grid` - diagram of the grid where the rolls of paper are located
///
/// # Returns
///
/// A [`u64`] the number of rolls of paper that can be accessed by a forklift  
///
/// # Examples
///          y
///          |
///          v
/// x->         0 1 2
///          0  - - @
///          1  - @ @
///          2  @ @ @
///
/// ```
/// let res = find_max_joltage(read_file_to_list("in.txt"));
/// ```
fn find_accessible_rolls(mut grid: Vec<Vec<char>>) -> u64 {
    let mut ris: u64 = 0;
    let roll_identifier = '@';
    let mut has_removed_at_least_one_roll = true;
    let mut new_grid: Vec<Vec<char>> = Vec::new();

    while has_removed_at_least_one_roll {
        has_removed_at_least_one_roll = false;
        if !new_grid.is_empty() {
            //if its not the first run
            grid = new_grid.clone();
            new_grid = Vec::new();
        }
        let grid_y_size = grid.len();
        for row in 0..grid_y_size {
            let grid_x_size = grid[row].len();
            new_grid.push(Vec::new());
            for col in 0..grid_x_size {
                //maybe not all the columns have the same size
                // counting the rolls in the adjacent positions of the roll at the position [row][col]
                if grid[row][col] != roll_identifier {
                    // if at the position there isnt a roll a skip
                    new_grid[row].push(grid[row][col]);
                    continue;
                }
                let mut rolls_around = 0;
                if row != 0 {
                    // i can check top row
                    if col != 0 && grid[row - 1][col - 1] == roll_identifier {
                        rolls_around += 1;
                    }
                    if grid[row - 1][col] == roll_identifier {
                        rolls_around += 1;
                    }
                    if col != grid_x_size - 1 && grid[row - 1][col + 1] == roll_identifier {
                        rolls_around += 1;
                    }
                }
                // same row as the roll row
                if col != 0 && grid[row][col - 1] == roll_identifier {
                    rolls_around += 1;
                }
                if col != grid_x_size - 1 && grid[row][col + 1] == roll_identifier {
                    rolls_around += 1;
                }
                if row != grid_y_size - 1 {
                    // row after the position im checking
                    if col != 0 && grid[row + 1][col - 1] == roll_identifier {
                        rolls_around += 1;
                    }
                    if grid[row + 1][col] == roll_identifier {
                        rolls_around += 1;
                    }
                    if col != grid_x_size - 1 && grid[row + 1][col + 1] == roll_identifier {
                        rolls_around += 1;
                    }
                }
                if rolls_around < 4 {
                    ris += 1;
                    new_grid[row].push('.'); //removing the roll for the next while
                    has_removed_at_least_one_roll = true;
                } else {
                    new_grid[row].push('@');
                }
            }
        }
    }
    // row index = y, col index = x

    return ris;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        find_accessible_rolls(read_file_to_matrix(input_file.to_string())?)
    );

    Ok(())
}
