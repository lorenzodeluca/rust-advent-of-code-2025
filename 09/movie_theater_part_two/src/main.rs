use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file line by line and put the integers in a matrix
///
/// for each row the 2 integers are coords [[x1,y1],[x2,y2], [row3] , ... ]
///
/// # Arguments
///
/// * path - path of the file to read.
///
/// # Returns
///
/// A [Result<Vec<Vec<i32>>>] with all the lines of the file.
///
/// # Examples
///
///  let input = read_file_to_matrix("in.txt"); 
fn read_file_to_matrix(path: String) -> io::Result<Vec<(i64, i64)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out: Vec<(i64, i64)> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(content) => {
                let mut parts = content.split(',');

                let x = parts.next().unwrap().parse::<i64>().unwrap();
                let y = parts.next().unwrap().parse::<i64>().unwrap();
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
/// * input - coords
///
/// # Returns
///
/// A [u64] representing the biggest area achievable
///
/// let res = find_biggest_rectangle(read_file_to_matrix("in.txt"));
/// ```
fn find_biggest_rectangle(input: Vec<(i64, i64)>) -> u64 {
    /*
    reasoning:
    idea 1: two pointers with pruning(it could not work in some case...)
    idea 2: brute force and some pruning if possible
    */
    let mut best_area: u64 = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let width = input[i].0.abs_diff(input[j].0)+1;
            let height = input[i].1.abs_diff(input[j].1)+1;
            let area = width * height;
            //print!("{},{}-{},{}-{}\n",input[i].0,input[i].1,input[j].0,input[j].1,area);
            //the rectangle is valid if all the corners are inside the allowed area
            if area > best_area && is_rectangle_valid(&input, i, j) {
                best_area = area;
            }
        }
    }
    best_area
}

fn is_rectangle_valid(input: &Vec<(i64, i64)>, i: usize, j: usize) -> bool {
    let x_min = min(input[i].0, input[j].0);
    let x_max = max(input[i].0, input[j].0);
    let y_min = min(input[i].1, input[j].1);
    let y_max = max(input[i].1, input[j].1);

    // 1. Algoritmo Ray-Casting per vedere se un punto (tile) è nel loop
    let mut inside = false;

    // Usiamo il centro della mattonella (x+0.5, y+0.5) per evitare ambiguità sui bordi
    let test_x = x_min as f64 + 0.5;
    let test_y = y_min as f64 + 0.5;
    let n = input.len();

    for k in 0..n {
        let p1 = input[k];
        let p2 = input[(k + 1) % n];
        if ((p1.1 as f64 > test_y) != (p2.1 as f64 > test_y)) &&
        (test_x < (p2.0 - p1.0) as f64 * (test_y - p1.1 as f64) / (p2.1 - p1.1) as f64 + p1.0 as f64) {
            inside = !inside;
        }
    }

    // 2. i 4 angoli devono essere dentro
    if !inside { return false; }

    // 3. Controllo collisioni: nessun lato del perimetro deve attraversare il rettangolo
    for k in 0..n {
        let p1 = input[k];
        let p2 = input[(k + 1) % n];
        let s_x_min = min(p1.0, p2.0);
        let s_x_max = max(p1.0, p2.0);
        let s_y_min = min(p1.1, p2.1);
        let s_y_max = max(p1.1, p2.1);

        if p1.0 == p2.0 {//segmento verticale
            if p1.0 > x_min && p1.0 < x_max && s_y_min < y_max && s_y_max > y_min { return false; }
        } else {//segmento orizzontale
            if p1.1 > y_min && p1.1 < y_max && s_x_min < x_max && s_x_max > x_min { return false; }
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";
    print!(
        "{}",
        find_biggest_rectangle(read_file_to_matrix(input_file.to_string())?)
    );
    Ok(())
}