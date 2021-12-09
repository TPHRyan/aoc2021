use crate::Error;
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

pub fn first_line(input: &str) -> Result<String, Error> {
    match input.lines().next() {
        Some(line) => Ok(String::from(line)),
        None => Err(Error::new("No data found for lanternfish simulation!")),
    }
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
    fn bit_lines_produces_correct_vectors() {
        let test_str = "11011\n01011";
        let expected_data: [[u8; 5]; 2] = [[1, 1, 0, 1, 1], [0, 1, 0, 1, 1]];

        let bit_lines = bit_lines(test_str);
        assert_eq!(expected_data.len(), bit_lines.len());
        for i in 0..expected_data.len() {
            let expected_line = expected_data[i];
            let bit_line = &bit_lines[i];
            assert_eq!(expected_line.len(), bit_line.len());
            for j in 0..expected_line.len() {
                assert_eq!(expected_line[j], bit_line[j]);
            }
        }
    }

    #[test]
    fn int_lines_splits_correct_times() {
        let test_str = String::from("28\n1423\n9043\n192");
        let split_str: Vec<Result<i32, ParseIntError>> = int_lines(&test_str).collect();
        assert_eq!(4, split_str.len());
    }
}
