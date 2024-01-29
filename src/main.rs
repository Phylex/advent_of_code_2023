use std::path::PathBuf;
use std::fs::File;
use clap::{arg, value_parser, Command, ArgAction};

mod day_1;
mod day_2;

fn main() {
    let command_line_argument = Command::new("day_1")
        .version("0.1")
        .author("Alexander Becker")
        .about("Solve the puzzles from advent of code 2023")
        .arg(arg!(<DAY> "the day the input was given for").required(true).value_parser(value_parser!(u32)))
        .arg(arg!(<INPUT>).help("Path to the input file").value_parser(value_parser!(PathBuf)).required(true))
        .arg(arg!(-t --two "set the program to work on part 2 of the problem").action(ArgAction::SetTrue))
        .get_matches();
    let day = command_line_argument.get_one::<u32>("DAY").expect("required Argument");
    let fpath = match command_line_argument.get_one::<PathBuf>("INPUT").expect("Required Argument").canonicalize() {
        Ok(fpath) => fpath,
        Err(err) => {
            println!("Cannot open file due to: {}", err);
            return;
        }
    };
    let file = match File::open(&fpath) {
        Err(why) => {
            println!("Cannot open file due to: {}", why);
            return;
        },
        Ok(file) => file,
    };
    let freader = std::io::BufReader::new(file);
    match day {
        1 => day_1::solve_day_1(freader, *command_line_argument.get_one("two").unwrap()),
        2 => day_2::solve_day_2(freader, *command_line_argument.get_one("two").unwrap()),
        _ => {
            println!("Not implemented or out of bounds");
        }
    }
}
