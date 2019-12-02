use clap::App;

mod day1;
mod day2;
mod input;

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("1.0")
        .author("Peter Thompson <peter.thompson@dunelm.org.uk>")
        .about("Advent of Code 2019 solutions")
        .subcommand(App::new("day1-1").about("Day 1: Part 1"))
        .subcommand(App::new("day1-2").about("Day 1: Part 2"))
        .subcommand(App::new("day2-1").about("Day 2: Part 1"))
        .get_matches();

    match matches.subcommand_name() {
        Some("day1-1") => day1::part1(input::as_string("day1").as_ref()),
        Some("day1-2") => day1::part2(input::as_string("day1").as_ref()),
        Some("day2-1") => day2::part1(input::parse_comma_separated_ints("day2")),
        None => println!("You need to specify a day to get a solution"),
        _ => println!("I don't understand :("),
    }
}
