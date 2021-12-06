use std::iter::Filter;
use std::num::ParseIntError;
use std::str::Lines;

const ASCII_ZERO_VALUE: u8 = '0' as u8;

pub fn bit_lines(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - ASCII_ZERO_VALUE).collect())
        .collect()
}

pub fn int_lines<'a>(
    input: &'a String,
) -> Box<dyn Iterator<Item = Result<i32, ParseIntError>> + 'a> {
    int_lines_radix(input, 10)
}

pub fn int_lines_radix<'a>(
    input: &'a String,
    radix: u32,
) -> Box<dyn Iterator<Item = Result<i32, ParseIntError>> + 'a> {
    Box::new(filter_newlines(input).map(move |slice| i32::from_str_radix(slice, radix)))
}

pub fn filter_newlines<'a>(input: &'a String) -> Filter<Lines<'_>, fn(&&'a str) -> bool> {
    input.lines().filter(|&slice| slice != "")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_lines_splits_correct_times() {
        let test_str = String::from("28\n1423\n9043\n192");
        let split_str: Vec<Result<i32, ParseIntError>> = int_lines(&test_str).collect();
        assert_eq!(4, split_str.len());
    }
}
