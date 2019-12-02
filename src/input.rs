use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn as_string(file: &str) -> String {
    let path = Path::new("./input").join(file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => (),
    }

    s
}

pub fn parse_comma_separated_ints(file: &str) -> Vec<i64> {
    if let Ok(ints) = as_string(file).split(',').map(str::parse::<i64>).collect() {
        return ints;
    }

    panic!("Couldn't parse input into a list of integers")
}
