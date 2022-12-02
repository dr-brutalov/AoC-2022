use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::env;
use std::fs;


///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    
    // File I/O
    let test_path: &str = "input/01_test.txt";
    //let input_path: &str = "input/01_input.txt";
    println!("In file {}", test_path);

    let contents = fs::read_to_string(test_path)
        .expect("Should have been able to read the file, didn't.");

    println!("With text:\n{contents}");

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
