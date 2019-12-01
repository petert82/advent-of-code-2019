use clap::App;

mod day1;
mod input;

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("1.0")
        .author("Peter Thompson <peter.thompson@dunelm.org.uk>")
        .about("Advent of Code 2019 solutions")
        .subcommand(App::new("day1-1").about("Day 1: Part 1"))
        .subcommand(App::new("day1-2").about("Day 1: Part 2"))
        .get_matches();

    match matches.subcommand_name() {
        Some("day1-1") => day1::part1(input::get_input("day1").as_ref()),
        Some("day1-2") => day1::part2(input::get_input("day1").as_ref()),
        None => println!("You need to specify a day to get a solution"),
        _ => println!("I don't understand :("),
    }
}
