use crate::prelude::*;
use regex::RegexBuilder;
use std::cmp;
use std::iter::zip;

pub struct AocDayOne;

impl AocDay for AocDayOne {
    fn get_day(&self) -> u8 {
        1
    }

    fn solve_part1(&self, input: &str) -> DayPuzzlePair {
        let mut sum = 0;
        let re = RegexBuilder::new(r"(?<lhs>\d+)\s+(?<rhs>\d+)")
            .build()
            .unwrap();
        let mut lhs_distances = Vec::new();
        let mut rhs_distances = Vec::new();
        for line in input.lines() {
            if let Some(distances) = re.captures(line) {
                lhs_distances.push(
                    distances["lhs"]
                        .parse::<usize>()
                        .expect("Could not parse into int"),
                );
                rhs_distances.push(
                    distances["rhs"]
                        .parse::<usize>()
                        .expect("Could not parse into int"),
                );
            }
        }
        lhs_distances.sort();
        rhs_distances.sort();
        for (lhs, rhs) in zip(lhs_distances.clone(), rhs_distances.clone()) {
            sum += cmp::max(lhs, rhs) - cmp::min(lhs, rhs);
        }
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(sum),
        }
    }

    fn solve_part2(&self, input: &str) -> DayPuzzlePair {
        let mut sum = 0;
        let re = RegexBuilder::new(r"(?<lhs>\d+)\s+(?<rhs>\d+)")
            .build()
            .unwrap();
        let mut lhs_distances = Vec::new();
        let mut rhs_distances = Vec::new();
        for line in input.lines() {
            if let Some(distances) = re.captures(line) {
                lhs_distances.push(
                    distances["lhs"]
                        .parse::<usize>()
                        .expect("Could not parse into int"),
                );
                rhs_distances.push(
                    distances["rhs"]
                        .parse::<usize>()
                        .expect("Could not parse into int"),
                );
            }
        }
        for lhs in lhs_distances.clone() {
            sum += lhs
                * (rhs_distances
                    .clone()
                    .into_iter()
                    .filter(|distance| &lhs == distance)
                    .collect::<Vec<_>>()
                    .len());
        }
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(sum),
        }
    }
}
