use crate::{Solution, SolutionPair};
use std::fs;

// Give our jolly little elves a struct
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: u64, // Since this is all we know about our elves so far, start here.
}

// Add the appropriate methods to the elf struct
impl Elf {
    fn add_calories(&mut self, value: u64) -> () {
        self.calories += value;
    }

    fn get_calories(&self) -> u64 {
        self.calories
    }
}

// File import function
pub fn day_one_solutions(input: String) -> SolutionPair {
    
    // Since the input data is separated by a `\newline` return, the `lines()` function is appropriate
    let mut lines = input.lines();

    // Start processing Elves calories in a vector
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf: Elf = Elf { calories: 0 };

    while let Some(line) = lines.next() {
        if line == "" {
            elves.push(elf);
            elf = Elf { calories: 0 };
        } else {
            let calories: u64 = line.parse().unwrap();
            elf.add_calories(calories);
        }
    }

    elves.push(elf);

    elves.sort();
    elves.reverse();

    let solution_one: u64 = elves[0].get_calories();
    let solution_two: u64 = 0;

    (Solution::U64(solution_one), Solution::U64(solution_two))

}

/*
\\ Testing for day 1 (currently broken, need to fix the PartialOrd problem) 
\\ Run with `cargo test 1`
#[test]
fn test() {
    // Set up the file I/O
    let test_path: &str = "input/01_test.txt";
    let input: String = fs::read_to_string(test_path)
        .expect("Unable to read the test file, yo.");

    let (solution_one, solution_two) = day_one_solutions(input);

    assert_eq!(solution_one, Solution::U64(24000));
    assert_eq!(solution_two, Solution::U64(0));
}
*/

///////////////////////////////////////////////////////////////////////////////

// Run with cargo run --release 1
pub fn solve() -> SolutionPair {
    
    // File I/O
    //let test_path: &str = "input/01_test.txt";
    let input_path: &str = "input/01_input.txt";
    //println!("In file {}", input_path);
    let input: String = fs::read_to_string(input_path)
        .expect("Should have been able to read the input file, didn't, yo.");

    day_one_solutions(input)

}
