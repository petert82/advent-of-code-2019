#[derive(Debug, PartialEq)]
enum Error {
    BadNumber,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::BadNumber
    }
}

pub fn part1(input: &str) {
    match total_fuel_for_modules(input) {
        Ok(total) => println!("The sum of the fuel requirements is: {}", total),
        Err(Error::BadNumber) => println!("One of the input masses could not be parsed"),
    }
}

fn total_fuel_for_modules(masses: &str) -> Result<i64, Error> {
    Ok(fuel_for_modules(masses)?.into_iter().sum())
}

fn fuel_for_modules(masses: &str) -> Result<Vec<i64>, Error> {
    masses.lines().map(|mass| fuel_for_module(mass)).collect()
}

fn fuel_for_module(mass: &str) -> Result<i64, Error> {
    let num_mass = mass.parse::<i64>()?;
    Ok((num_mass / 3) - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module("12"), Ok(2));
        assert_eq!(fuel_for_module("14"), Ok(2));
        assert_eq!(fuel_for_module("1969"), Ok(654));
        assert_eq!(fuel_for_module("100756"), Ok(33583));
    }

    #[test]
    fn test_total_fuel_for_modules() {
        assert_eq!(total_fuel_for_modules("12"), Ok(2));
        assert_eq!(total_fuel_for_modules("12\n14"), Ok(4));
        assert_eq!(total_fuel_for_modules("12\n14\n1969"), Ok(658));
    }
}
