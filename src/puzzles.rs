use crate::prelude::*;

#[derive(Debug)]
pub struct DayPuzzlePair {
    pub input: String,
    pub output: Option<String>,
}

impl DayPuzzlePair {
    pub fn is_valid_solution(&self, solution: String) -> bool {
        self.input.len() > 0 && self.output.clone().unwrap_or_default().trim() == solution.trim()
    }
}

pub trait AocDay {
    fn get_day(&self) -> u8;
    fn get_part1_example(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P1_EXAMPLE_IN))
            .expect("Could not read part 1 example input");
        let output = std::fs::read_to_string(format!("{}/{}", day_path, P1_EXAMPLE_OUT)).ok();
        DayPuzzlePair { input, output }
    }
    fn get_part2_example(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P2_EXAMPLE_IN))
            .expect("Could not read part 2 example input");
        let output = std::fs::read_to_string(format!("{}/{}", day_path, P2_EXAMPLE_OUT)).ok();
        DayPuzzlePair { input, output }
    }
    fn get_part1(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P1_IN))
            .expect("Could not read part 1 input");
        DayPuzzlePair {
            input,
            output: None,
        }
    }
    fn get_part2(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P2_IN))
            .expect("Could not read part 2 input");
        DayPuzzlePair {
            input,
            output: None,
        }
    }
    fn solve_part1(&self, input: &str) -> DayPuzzlePair;
    fn solve_part2(&self, input: &str) -> DayPuzzlePair;
    fn validate_part1(&self) {
        println!("Part1: ");
        let example = self.get_part1_example();
        let example_solution = self.solve_part1(&example.input);
        if example_solution.is_valid_solution(example.output.clone().unwrap_or_default()) {
            println!("Valid solution!");
            println!("Example solution: {}", example_solution.output.unwrap());
        } else {
            eprintln!("Example solution is not valid!");
            eprintln!("Example solution: {}", example_solution.output.unwrap());
            eprintln!("Example expected: {}", example.output.unwrap());
            std::process::exit(1);
        }
    }
    fn validate_part2(&self) {
        println!("Part2: ");
        let example = self.get_part2_example();
        let example_solution = self.solve_part2(&example.input);
        if example_solution.is_valid_solution(example.output.clone().unwrap_or_default()) {
            println!("Valid solution!");
            println!("Example solution: {}", example_solution.output.unwrap());
        } else {
            eprintln!("Example solution is not valid!");
            eprintln!("Example solution: {}", example_solution.output.unwrap());
            eprintln!("Example expected: {}", example.output.unwrap());
            std::process::exit(1);
        }
    }
    fn validate(&self) {
        self.validate_part1();
        self.validate_part2();
    }
}
