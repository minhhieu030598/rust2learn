use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn native_read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    // Solution 1
    let lines : Vec<String> = native_read_lines("src/new_file.txt");
    for line in lines {
        println!("{}", line)
    }

    // Solution 2
    // let lines : Vec<String> = read_lines("src/new_file.txt");
    // for line in lines {
    //     println!("{:?}", line)
    // }


}



// fn main() {
//     // File hosts.txt must exist in the current path
//     if let Ok(lines) = read_lines("./hosts.txt") {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines.flatten() {
//             println!("{}", line);
//         }
//     }
// }
//
// // The output is wrapped in a Result to allow matching on errors.
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }