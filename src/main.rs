use clap::Parser;
use days::day01::AocDayOne;
use puzzles::AocDay;
use std::path::PathBuf;

mod constants;
mod days;
mod parse;
mod prelude;
mod puzzles;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Args {
    /// Day to solve
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=25), default_value_t = 0)]
    day: u8,

    /// Part to solve
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=25))]
    part: u8,

    /// Path to data dir
    #[arg(long)]
    data_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let data_path = args.data_path.unwrap_or(PathBuf::from("./data"));
    let days_path = format!("{}/days", data_path.display());
    std::fs::create_dir_all(&days_path).unwrap();
    std::env::set_var("DATA_PATH", data_path.display().to_string());
    let mut days: Vec<&dyn AocDay> = vec![&AocDayOne];

    if let Some(day) = days.iter().find(|d| d.get_day() == args.day) {
        day.validate()
    } else {
        if args.day == 0 && days.len() > 0 {
            println!(
                "Days available: {}",
                days.iter()
                    .map(|d| d.get_day().to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            std::process::exit(1);
        } else if args.day > 0 {
            eprintln!("Day {} is not yet implemented!", args.day);
            std::process::exit(1);
        } else {
            eprintln!("No days available!");
            eprintln!("Directory structure:");
            eprintln!("data/");
            eprintln!("    days/");
            eprintln!(
                "        {:02}/",
                days.iter().map(|d| d.get_day()).max().unwrap_or(1)
            );
            eprintln!("            p1_example_in.txt");
            eprintln!("            p1_example_out.txt");
            eprintln!("            p2_example_in.txt");
            eprintln!("            p2_example_out.txt");
            eprintln!("            p1_in.txt");
            eprintln!("            p2_in.txt");
            std::process::exit(1);
        }
    }
}
