use std::fs;
use std::path::{self, PathBuf};

pub fn read_input(data_path: &PathBuf, day: u8, part: u8, example: bool) -> String {
    let mut part_path = data_path.clone();
    let mut input_filename = "input.txt";
    part_path.push("days/");
    part_path.push(day.to_string());
    part_path.push("parts/");
    part_path.push(part.to_string());
    if example {
        input_filename = "example_input.txt";
    }
    part_path.push(input_filename);
    let abs_path = path::absolute(part_path.clone()).expect(&format!(
        "Could not get absolute path for {}",
        part_path.display()
    ));
    println!("Reading input from {}", abs_path.display());
    fs::read_to_string(part_path).expect("Could not open input file")
}
