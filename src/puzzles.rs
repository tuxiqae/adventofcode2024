use crate::prelude::*;

#[derive(Debug)]
pub struct DayPuzzlePair {
    pub input: String,
    pub output: Option<usize>,
}

impl DayPuzzlePair {
    pub fn is_valid_solution(&self, solution: usize) -> bool {
        self.input.len() > 0 && self.output.clone().unwrap_or_default() == solution
    }
}

pub trait AocDay {
    fn get_day(&self) -> u8;
    fn get_part1_example(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P1_EXAMPLE_IN))
            .expect("Could not read part 1 example input");
        let output = Some(
            std::fs::read_to_string(format!("{}/{}", day_path, P1_EXAMPLE_OUT))
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
        );
        DayPuzzlePair { input, output }
    }
    fn get_part2_example(&self) -> DayPuzzlePair {
        let data_path = std::env::var("DATA_PATH").expect("DATA_PATH not set");
        let day_path = format!("{}/days/{:02}", data_path, self.get_day());
        let input = std::fs::read_to_string(format!("{}/{}", day_path, P2_EXAMPLE_IN))
            .expect("Could not read part 2 example input");
        let output = Some(
            std::fs::read_to_string(format!("{}/{}", day_path, P2_EXAMPLE_OUT))
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
        );
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
    fn validate_part(&self, part_num: u8) {
        println!("Part #{}: ", part_num);
        let mut example: DayPuzzlePair;
        let mut example_solution: DayPuzzlePair;
        if part_num == 1 {
            example = self.get_part1_example();
            example_solution = self.solve_part1(&example.input);
        } else if part_num == 2 {
            example = self.get_part2_example();
            example_solution = self.solve_part2(&example.input);
        } else {
            eprintln!("Part \"{}\" is not supported", part_num);
            std::process::exit(1);
        }
        if example_solution.is_valid_solution(example.output.clone().unwrap_or_default()) {
            println!("Valid solution!");
            println!("  solution: \"{}\"", example_solution.output.unwrap());
        } else {
            eprintln!("Example solution is invalid!");
            eprintln!("  Expected \"{}\"", example_solution.output.unwrap());
            eprintln!("  Received: \"{}\"", example.output.unwrap());
            if part_num == 1 {
                std::process::exit(1);
            }
        }
        println!();
    }

    fn validate(&self) {
        self.validate_part(1);
        self.validate_part(2);
    }

    fn solve(&self) {
        self.validate();
        println!("=================================");
        let solution_part1 = self.solve_part1(&self.get_part1().input).output.unwrap();
        let solution_part2 = self.solve_part2(&self.get_part2().input).output.unwrap();
        println!(
            "Solutions:\n  Part 1: \"{}\"\n  Part 2: \"{}\"",
            solution_part1, solution_part2
        );
        println!("=================================");
    }
}
