#[derive(Debug, PartialEq)]
enum Error {
    BadNumber,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::BadNumber
    }
}

type FuelCalculator = fn(i64) -> i64;

pub fn part1(input: &str) {
    match total_fuel_for_modules(input, fuel_for_mass) {
        Ok(total) => println!("The sum of the fuel requirements is: {}", total),
        Err(Error::BadNumber) => println!("One of the input masses could not be parsed"),
    }
}

fn total_fuel_for_modules(masses: &str, calculator: FuelCalculator) -> Result<i64, Error> {
    Ok(fuel_for_modules(masses, calculator)?.into_iter().sum())
}

fn fuel_for_modules(masses: &str, calculator: FuelCalculator) -> Result<Vec<i64>, Error> {
    Ok(masses
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(calculator)
        .collect())
}

fn fuel_for_mass(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel
}

pub fn part2(input: &str) {
    match total_fuel_for_modules(input, fuel_for_module_recursive) {
        Ok(total) => println!("The sum of the fuel requirements is: {}", total),
        Err(Error::BadNumber) => println!("One of the input masses could not be parsed"),
    }
}

fn fuel_for_module_recursive(module_mass: i64) -> i64 {
    let mut tot = 0;
    let mut fuel = fuel_for_mass(module_mass);

    while fuel > 0 {
        tot = tot + fuel;
        fuel = fuel_for_mass(fuel);
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_fuel_for_module_recursive() {
        assert_eq!(fuel_for_module_recursive(14), 2);
        assert_eq!(fuel_for_module_recursive(1969), 966);
        assert_eq!(fuel_for_module_recursive(100756), 50346);
    }

    #[test]
    fn test_total_fuel_for_modules() {
        assert_eq!(total_fuel_for_modules("12", fuel_for_mass), Ok(2));
        assert_eq!(total_fuel_for_modules("12\n14", fuel_for_mass), Ok(4));
        assert_eq!(
            total_fuel_for_modules("12\n14\n1969", fuel_for_mass),
            Ok(658)
        );

        assert_eq!(
            total_fuel_for_modules("14", fuel_for_module_recursive),
            Ok(2)
        );
        assert_eq!(
            total_fuel_for_modules("14\n1969", fuel_for_module_recursive),
            Ok(968)
        );
    }
}
