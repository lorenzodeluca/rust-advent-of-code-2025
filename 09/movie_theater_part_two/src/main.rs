use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file line by line and put the integers in a matrix
///
/// for each row the 2 integers are coords [[x1,y1],[x2,y2], [row3] , ... ]
///
/// # Arguments
///
/// * `path` - path of the file to read.
///
/// # Returns
///
/// A [`Result<Vec<Vec<i32>>>`] with all the lines of the file.
///
/// # Examples
///
/// ```
/// let input = read_file_to_matrix("in.txt");
/// ```
fn read_file_to_matrix(path: String) -> io::Result<Vec<(u64, u64)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out: Vec<(u64, u64)> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let mut parts = content.split(',');

                let x = parts.next().unwrap().parse::<u64>().unwrap();
                let y = parts.next().unwrap().parse::<u64>().unwrap();
                out.push((x, y));
            }
            Err(e) => eprintln!("Error reading the file: {}", e),
        }
    }

    Ok(out)
}

/// find_biggest_rectangle
///
/// find the biggest rectangle that has two opposite angles in two input coords
///
/// # Arguments
///
/// * `input` - coords
///
/// # Returns
///
/// A [`u64`] representing the biggest area achievable
///
/// let res = find_biggest_rectangle(read_file_to_matrix("in.txt"));
/// ```
fn find_biggest_rectangle(mut input: Vec<(u64, u64)>) -> u64 {
     /*
        reasoning:
            idea 1: two pointers with pruning(it could not work in some case...)
            idea 2: brute force and some pruning if possible
     */
    let mut best_area: u64 = 0;
    for i in 0..input.len() {
        for j in (0..input.len()).rev() {
            //todo if pruning: min and max y from y to j
            let width = input[i].0.abs_diff(input[j].0)+1;
            let height = input[i].1.abs_diff(input[j].1)+1;
            let area = width * height;

            //print!("{},{}-{},{}-{}\n",input[i].0,input[i].1,input[j].0,input[j].1,area);
            
            //the rectangle is valid if all the corners are inside the allowed area
            if area>best_area && is_rectangle_valid(&input,i,j) {
                best_area = area;
            }
        }
    }
    best_area
}

fn is_rectangle_valid(input: &Vec<(u64, u64)>, i: usize, j: usize) -> bool {
    let mut points_to_check: Vec<(u64, u64)> = Vec::new();
    points_to_check.push((input[i].0,input[j].1));
    points_to_check.push((input[j].0,input[i].1));
    for point_to_check in points_to_check{
        let mut top_left_valid= false;
        let mut top_right_valid= false;
        let mut bottom_left_valid= false;
        let mut bottom_right_valid= false;
        for input_point in input{
            if input_point.1>=point_to_check.1&&input_point.0<=point_to_check.0 {
                top_left_valid=true;
            }
            if input_point.1>=point_to_check.1&&input_point.0>=point_to_check.0 {
                top_right_valid=true;
            }
            if input_point.1<=point_to_check.1&&input_point.0<=point_to_check.0 {
                bottom_left_valid=true;
            }
            if input_point.1<=point_to_check.1&&input_point.0>=point_to_check.0 {
                bottom_right_valid=true;
            }
            if top_left_valid && top_right_valid && bottom_left_valid && bottom_right_valid{
                break;
            }
        }
        if !top_left_valid || !top_right_valid || !bottom_left_valid ||bottom_right_valid{
            return false;
        }
    }
    return true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        find_biggest_rectangle(read_file_to_matrix(input_file.to_string())?)
    );

    Ok(())
}
