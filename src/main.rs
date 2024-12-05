use clap::Parser;
use std::path::PathBuf;

mod days;
mod parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Args {
    /// Day to solve
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part to solve
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=25))]
    part: u8,

    /// Path to data dir
    #[arg(long)]
    data_path: Option<PathBuf>,

    /// Example run
    #[arg(long, default_value_t = true)]
    example: bool,
}

fn main() {
    let args = Args::parse();

    let data_path = match args.data_path {
        Some(path) => path,
        None => PathBuf::from("./data"),
    };

    let day = args.day;
    match day {
        1 => days::day01::solve(&data_path, args.day, args.part, args.example),
        2 => days::day02::solve(),
        _ => {
            eprintln!("Day {} is not yet implemented!", day);
            std::process::exit(1);
        }
    }
}
