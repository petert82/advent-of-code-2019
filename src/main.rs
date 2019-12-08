use clap::App;

mod day1;
mod day2;
mod day3;
mod day4;
mod input;

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("1.0")
        .author("Peter Thompson <peter.thompson@dunelm.org.uk>")
        .about("Advent of Code 2019 solutions")
        .subcommand(App::new("day1-1").about("Day 1: Part 1"))
        .subcommand(App::new("day1-2").about("Day 1: Part 2"))
        .subcommand(App::new("day2-1").about("Day 2: Part 1"))
        .subcommand(App::new("day2-2").about("Day 2: Part 2"))
        .subcommand(App::new("day3-1").about("Day 3: Part 1"))
        .subcommand(App::new("day3-2").about("Day 3: Part 2"))
        .subcommand(App::new("day4-1").about("Day 4: Part 1"))
        .get_matches();

    match matches.subcommand_name() {
        Some("day1-1") => day1::part1(input::as_string("day1").as_ref()),
        Some("day1-2") => day1::part2(input::as_string("day1").as_ref()),
        Some("day2-1") => day2::part1(input::parse_comma_separated_ints("day2")),
        Some("day2-2") => day2::part2(input::parse_comma_separated_ints("day2")),
        Some("day3-1") => day3::part1(input::as_string("day3").as_ref()),
        Some("day3-2") => day3::part2(input::as_string("day3").as_ref()),
        Some("day4-1") => day4::part1(130254, 678275),
        None => println!("You need to specify a day to get a solution"),
        _ => println!("I don't understand :("),
    }
}
