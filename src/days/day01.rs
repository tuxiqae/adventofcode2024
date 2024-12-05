use crate::prelude::*;

pub struct AocDayOne;

impl AocDay for AocDayOne {
    fn get_day(&self) -> u8 {
        1
    }

    fn solve_part1(&self, input: &str) -> DayPuzzlePair {
        let mut sum = 0;
        for line in input.lines() {
            sum += line.parse::<u32>().unwrap();
        }
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(sum.to_string()),
        }
    }

    fn solve_part2(&self, input: &str) -> DayPuzzlePair {
        let mut sum = 0;
        for line in input.lines() {
            sum += line.parse::<u32>().unwrap();
        }
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(sum.to_string()),
        }
    }
}
