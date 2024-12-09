use crate::prelude::*;

pub struct AocDayTwo;

fn validate_report(report: &str) -> usize {
    let valid_delta: std::ops::RangeInclusive<isize> = 1..=3;
    println!("Validating report {report}");
    let mut report_vec: Vec<isize> = report
        .split_ascii_whitespace()
        .map(|l| l.parse::<isize>().unwrap())
        .collect();
    if report_vec.get(0) > report_vec.get(1) {
        println!("Report is not ascending. Reversing");
        report_vec.reverse(); // Make sure that the order is either ascending or unordered
    }
    report_vec
        .windows(2)
        .map(|w| valid_delta.contains(&(w.get(1).unwrap() - w.get(0).unwrap())))
        .all(|b| b) as usize
}

impl AocDay for AocDayTwo {
    fn get_day(&self) -> u8 {
        2
    }

    fn solve_part1(&self, input: &str) -> DayPuzzlePair {
        let mut valid_reports = 0;
        let valid_delta: std::ops::RangeInclusive<isize> = 1..=3;
        let mut report_vec: Vec<isize>;
        for report in input.lines() {
            // println!("Validating report #{idx}");
            report_vec = report
                .split_ascii_whitespace()
                .map(|l| l.parse::<isize>().unwrap())
                .collect();
            if report_vec.get(0) > report_vec.get(1) {
                // println!("Report is not ascending. Reversing");
                report_vec.reverse(); // Make sure that the order is either ascending or unordered
            }
            valid_reports += report_vec
                .windows(2)
                .map(|w| valid_delta.contains(&(w.get(1).unwrap() - w.get(0).unwrap())))
                .all(|b| b) as usize
        }
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(valid_reports),
        }
    }

    fn solve_part2(&self, input: &str) -> DayPuzzlePair {
        let sum = 0;
        DayPuzzlePair {
            input: input.to_string(),
            output: Some(sum),
        }
    }
}
