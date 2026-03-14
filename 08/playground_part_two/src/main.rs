use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// reads a file line by line and put the integers in a matrix
///
/// for each row the 3 integers are coords [[x1,y1,z1],[x2,y2,z2], [row3] , ... ]
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
fn read_file_to_matrix(path: String) -> io::Result<Vec<Vec<u64>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => out.push(content.split(',').map(|s| s.parse::<u64>().unwrap()).collect()),
            Err(e) => eprintln!("Error reading the file: {}", e),
        }
    }

    Ok(out)
}

/// solve
///
/// connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits
///
/// # Arguments
///
/// * `input` - coords of the junction boxes
///
/// # Returns
///
/// A [`i32`] the product of the 3 biggest groups
///
/// let res = solve(read_file_to_matrix("in.txt"));
/// ```
fn solve(mut input: Vec<Vec<u64>>) -> u64 {
     /*
        reasoning:
        nodes = junction boxes
        for each node A i need to know only the closest one B
        the O(n*n) solution is obvious, i think that i can do better
        idea 1: having some kind of 3d rappresentation of the nodes in such a way that it allows me to explore the space close to a node so that i dont need to iterate over all the nodes to find the closest one(a graph  in which i connect a node only to the closest one). Maybe its not convinient to initialize such structure, complexity speaking
        idea 2: having 3 vectors(one for each dimension) in which the nodes are sorted by one axis so that i can look in each direction for the nodes closest to the one im computing 
     
        i choose to go for the basic solution... 
     */
    
    //step 1 calculating the distances
    let mut closest: Vec<(i64, usize, usize)> = Vec::new();
    let mut res = 0;
    for i in 0..input.len() {
        for j in  (i+1)..input.len() {
            let dx:i64 = input[i][0] as i64-input[j][0] as i64;
            let dy:i64 = input[i][1] as i64-input[j][1] as i64;
            let dz:i64 = input[i][2] as i64-input[j][2] as i64;
            let distance = dx*dx + dy*dy + dz*dz;
            closest.push((distance,i,j));
        }
    }
    closest.sort();

    //step 2 calculanting the groups
    let mut circuit_size: Vec<u64> = Vec::new();
    let mut connections = 0;
    //print!("closest: {:?}\n\n", closest);
    for (_,node,closest) in closest{
        //connecting node and closest
        if input[node].len() == 4 && input[closest].len() == 4 && input[node][3]!=input[closest][3]{ //merging the circuits
            //print!("merging {:?} to {:?}\n",input[node],input[closest]);
            //deleting closest circuit
            //adding all the closest circuit nodes into node circuit
            let node_circuit_id = input[node][3];
            let closest_circuit_id = input[closest][3];
            for i in 0..input.len() {//moving nodes from closest circuit to node circuit
                if input[i].len()==4 && input[i][3]==closest_circuit_id{
                    input[i][3]=node_circuit_id;
                }
            }
            circuit_size[node_circuit_id as usize] += circuit_size[closest_circuit_id as usize];
            circuit_size[closest_circuit_id as usize] = 0;
            res = input[node][0]*input[closest][0];
        }else if input[node].len() == 4 { //node is already in a circuit
            let node_circuit_id = input[node][3];
            if input[closest].len() == 3{ // closest isnt already in a circuit
                //adding closest to the same circuit
                input[closest].push(node_circuit_id);
                circuit_size[node_circuit_id as usize] += 1;
                res = input[node][0]*input[closest][0];
                //print!("connecting1 {:?} to {:?}\n",input[node],input[closest]);
            }
        }else if input[closest].len() == 4 { //closest is already in a circuit
            let closest_circuit_id = input[closest][3];
            if input[node].len() == 3{ // node isnt already in a circuit
                //adding node to the same circuit
                input[node].push(closest_circuit_id);
                circuit_size[closest_circuit_id as usize] += 1;
                res = input[node][0]*input[closest][0];
                //print!("connecting2 {:?} to {:?}\n",input[node],input[closest]);
            }
        }else{//creating a new circuit
            let circuit_id=circuit_size.len();
            circuit_size.push(2);
            input[node].push(circuit_id as u64);
            input[closest].push(circuit_id as u64);
            res = input[node][0]*input[closest][0];
            //print!("creating {:?} to {:?}\n",input[node],input[closest]);
        }
    }
    //step 3 Multiplying together the sizes of the three largest circuits
    circuit_size.sort();
    //print!("input: {:?}\n\n", input);
    //print!("circuit_size: {:?}", circuit_size);
    return res;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "in.txt";

    print!(
        "{}",
        solve(read_file_to_matrix(input_file.to_string())?)
    );

    Ok(())
}
