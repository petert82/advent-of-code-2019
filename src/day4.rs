pub fn part1(start: i64, end: i64) {
    let tot = (start..=end).fold(0, |count, num| {
        if is_valid_passcode(num) {
            return count + 1;
        }
        count
    });
    println!("{} values are possible passcodes", tot);
}

fn is_valid_passcode(test_code: i64) -> bool {
    let code = format!("{}", test_code);

    if code.len() != 6 {
        return false;
    }

    let mut prev_digit = 0;
    let mut has_double = false;
    for c in code.chars() {
        let digit = c.to_digit(10).unwrap();
        if digit < prev_digit {
            return false;
        }
        if digit == prev_digit {
            has_double = true;
        }
        prev_digit = digit;
    }

    has_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_passcode() {
        assert_eq!(false, is_valid_passcode(12345)); // Too short
        assert_eq!(false, is_valid_passcode(223450)); // Descending pair
        assert_eq!(false, is_valid_passcode(123789)); // No double
        assert_eq!(false, is_valid_passcode(135679)); // No double
        assert_eq!(true, is_valid_passcode(122345));
        assert_eq!(true, is_valid_passcode(111123));
        assert_eq!(true, is_valid_passcode(335679));
        assert_eq!(true, is_valid_passcode(111111));
    }
}
