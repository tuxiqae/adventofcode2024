use crate::parse::read_input;
use std::env;
use std::path::PathBuf;

pub fn calculate(input_text: &str) -> usize {
    input_text.len() + 10
}

pub fn main(data_path: &PathBuf, day: u8, part: u8, example: bool) {
    println!("Running day 1 solution!, cwd={:?}", env::current_dir());
    let solution = solve(&data_path, day, part, example);
    println!("Solution: {}", solution);

    println!("With text:\n{contents}");
}

fn solve(data_path: &PathBuf, day: u8, part: u8, example: bool) {
    let contents = read_input(&data_path, day, part, example);
    let solution = calculate(&contents);
    assert!(solution);
    solution
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn check_solution {
        solve()
    }
}
*/

/*
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
*/
