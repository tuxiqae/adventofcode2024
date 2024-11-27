use clap::{Arg, Command};

mod days;

fn main() {
    let matches = Command::new("Day Selector")
        .arg(
            Arg::new("day")
                .help("Select a day between 1 and 25")
                .required(true)
                .value_parser(clap::value_parser!(u8).range(1..=25)),
        )
        .get_matches();

    let day = *matches.get_one::<u8>("day").expect("Required argument");

    match day {
        1 => days::day01::main::main(),
        2 => days::day02::main::main(),
        _ => {
            eprintln!("Day {} is not yet implemented!", day);
            std::process::exit(1);
        }
    }
}
