pub fn part1(start: i64, end: i64) {
    let tot = (start..=end).fold(0, |count, num| {
        if is_valid_part1_passcode(num) {
            return count + 1;
        }
        count
    });
    println!("{} values are possible passcodes", tot);
}

fn is_valid_part1_passcode(test_code: i64) -> bool {
    if test_code < 100_000 || test_code > 999_999 {
        return false;
    }
    let code = format!("{}", test_code);

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

pub fn part2(start: i64, end: i64) {
    let tot = (start..=end).fold(0, |count, num| {
        if is_valid_part2_passcode(num) {
            return count + 1;
        }
        count
    });
    println!("{} values are possible passcodes", tot);
}

fn is_valid_part2_passcode(test_code: i64) -> bool {
    if test_code < 100_000 || test_code > 999_999 {
        return false;
    }
    let code = format!("{}", test_code);

    let mut prev_digit = 0;
    let mut curr_run_len = 1;
    let mut has_double = false;
    for (i, c) in code.char_indices() {
        let digit = c.to_digit(10).unwrap();
        if digit < prev_digit {
            return false;
        }
        if digit == prev_digit {
            curr_run_len = curr_run_len + 1;
        }
        if curr_run_len == 2 {
            if digit != prev_digit || i == (code.len() - 1) {
                // Run is ending if curr digit doesn't match prev, or we are at the end of the string
                has_double = true;
            }
        }
        if digit != prev_digit {
            curr_run_len = 1;
        }
        prev_digit = digit;
    }

    has_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_part1_passcode() {
        assert_eq!(false, is_valid_part1_passcode(12345)); // Too short
        assert_eq!(false, is_valid_part1_passcode(223450)); // Descending pair
        assert_eq!(false, is_valid_part1_passcode(123789)); // No double
        assert_eq!(false, is_valid_part1_passcode(135679)); // No double
        assert_eq!(true, is_valid_part1_passcode(122345));
        assert_eq!(true, is_valid_part1_passcode(111123));
        assert_eq!(true, is_valid_part1_passcode(335679));
        assert_eq!(true, is_valid_part1_passcode(111111));
    }

    #[test]
    fn test_is_valid_part2_passcode() {
        assert_eq!(false, is_valid_part2_passcode(12345)); // Too short
        assert_eq!(false, is_valid_part2_passcode(223450)); // Descending pair
        assert_eq!(false, is_valid_part2_passcode(123789)); // No double
        assert_eq!(false, is_valid_part2_passcode(135679)); // No double
        assert_eq!(false, is_valid_part2_passcode(111123)); // Only double is in long run
        assert_eq!(false, is_valid_part2_passcode(123444)); // Only double is in long run
        assert_eq!(false, is_valid_part2_passcode(111111)); // Only double is in long run
        assert_eq!(true, is_valid_part2_passcode(112233));
        assert_eq!(true, is_valid_part2_passcode(111122));
        assert_eq!(true, is_valid_part2_passcode(112222));
        assert_eq!(true, is_valid_part2_passcode(122345));
        assert_eq!(true, is_valid_part2_passcode(335679));
    }
}
