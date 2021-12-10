use super::seven_segment::*;
use crate::{Error, Result};
use std::str::FromStr;

pub fn unscramble_outputs(signals_and_outputs: &Vec<(Vec<u8>, OutputDisplay)>) -> Result<Vec<u32>> {
    signals_and_outputs
        .iter()
        .map(|(signals, output_display)| unscramble_output(signals, output_display))
        .collect()
}

fn unscramble_output(signals: &Vec<u8>, output_display: &OutputDisplay) -> Result<u32> {
    let [one, four] = find_decoding_digits(signals)?;
    let output_string = output_display
        .digits
        .iter()
        .map(|&digit| unscramble_digit(digit, one, four))
        .fold(String::new(), |mut s, c| {
            s.push(c);
            s
        });
    Ok(u32::from_str(&output_string)?)
}

fn unscramble_digit(digit: u8, one: u8, four: u8) -> char {
    match count_segments(digit) {
        2 => '1',
        3 => '7',
        4 => '4',
        5 => match count_segments(digit & one) {
            1 => match count_segments(digit & four) {
                2 => '2',
                3 => '5',
                _ => '?',
            },
            2 => '3',
            _ => '?',
        },
        6 => match count_segments(digit & one) {
            1 => '6',
            2 => match count_segments(digit & four) {
                3 => '0',
                4 => '9',
                _ => '?',
            },
            _ => '?',
        },
        7 => '8',
        _ => '?',
    }
}

fn find_decoding_digits(signals: &Vec<u8>) -> Result<[u8; 2]> {
    let digits: [Option<u8>; 2] = signals.iter().fold(
        [None; 2],
        |[found_one, found_four], &digit| match count_segments(digit) {
            2 => [Some(digit), found_four],
            4 => [found_one, Some(digit)],
            _ => [found_one, found_four],
        },
    );
    match digits[..] {
        [Some(one), Some(four)] => Ok([one, four]),
        _ => Err(Box::new(Error::new("Could not find both digits 1 and 4!"))),
    }
}
