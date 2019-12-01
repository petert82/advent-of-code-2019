use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("1.0")
        .author("Peter Thompson <peter.thompson@dunelm.org.uk>")
        .about("Advent of Code 2019 solutions")
        .subcommand(App::new("day1").about("Day 1"))
        .get_matches();

    match matches.subcommand_name() {
        Some("day1") => println!("This is day 1"),
        None => println!("You need to specify a day to get a solution"),
        _ => println!("I don't understand :("),
    }
}
