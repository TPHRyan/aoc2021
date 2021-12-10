use crate::common::split_lines_on;
use crate::{Error, Result};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct OutputDisplay {
    pub digits: Vec<u8>,
}

impl OutputDisplay {
    pub fn from(digits: Vec<u8>) -> OutputDisplay {
        OutputDisplay { digits }
    }

    pub fn from_str(s: &str) -> Result<OutputDisplay> {
        let segments: Vec<u8> = s
            .split_whitespace()
            .map(|word| signal_to_digit(word))
            .collect::<Result<Vec<u8>>>()?;
        Ok(OutputDisplay::from(segments))
    }

    pub fn unique_digits(&self) -> Vec<&u8> {
        self.digits
            .iter()
            .filter(|&seg| has_unique_segment_count(*seg))
            .collect()
    }
}

impl Display for OutputDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .map(|segments| format!("{}", segments))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
fn bit_to_segment_letter(bit: u8) -> char {
    match bit {
        0b1000000 => 'a',
        0b0100000 => 'b',
        0b0010000 => 'c',
        0b0001000 => 'd',
        0b0000100 => 'e',
        0b0000010 => 'f',
        0b0000001 => 'g',
        _ => '?',
    }
}

fn segment_letter_to_bit(letter: char) -> Result<u8> {
    match letter {
        'a' => Ok(0b1000000),
        'b' => Ok(0b0100000),
        'c' => Ok(0b0010000),
        'd' => Ok(0b0001000),
        'e' => Ok(0b0000100),
        'f' => Ok(0b0000010),
        'g' => Ok(0b0000001),
        _ => Err(Box::new(Error::new(&format!("Unknown char {}!", letter)))),
    }
}

#[cfg(test)]
pub fn digit_to_signal(digit: u8) -> String {
    let mut signal = String::new();
    let mut remainder = digit;
    while remainder != 0 {
        let previous = remainder;
        remainder &= remainder - 1;
        signal = format!("{}{}", bit_to_segment_letter(previous - remainder), signal)
    }
    signal
}

pub fn signal_to_digit(signal: &str) -> Result<u8> {
    let digit: Vec<u8> = signal
        .chars()
        .map(|c| segment_letter_to_bit(c))
        .collect::<Result<Vec<u8>>>()?;
    Ok(digit.iter().fold(0, |acc, bit| acc | bit))
}

pub fn signals_to_digits(signals: &[&str]) -> Result<Vec<u8>> {
    signals
        .iter()
        .map(|signal| signal_to_digit(signal))
        .collect()
}

pub fn signal_output_lines_to_signals_and_output(
    lines: &str,
) -> Result<Vec<(Vec<u8>, OutputDisplay)>> {
    Ok(split_lines_on(lines, " | ")
        .filter_map(|parts| match parts[..] {
            [signal_str, output_str] => Some((
                signal_str.split_whitespace().collect::<Vec<&str>>(),
                output_str,
            )),
            _ => None,
        })
        .map(
            |(signal_vec, output_str)| match signals_to_digits(&signal_vec) {
                Ok(digits) => match OutputDisplay::from_str(output_str) {
                    Ok(display) => Ok((digits, display)),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        )
        .collect::<Result<Vec<(Vec<u8>, OutputDisplay)>>>()?)
}

pub fn count_segments(digit: u8) -> u8 {
    let mut count = 0;
    let mut remainder = digit;
    while remainder != 0 {
        remainder &= remainder - 1;
        count += 1;
    }
    count
}

pub fn has_unique_segment_count(digit: u8) -> bool {
    match count_segments(digit) {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::sub::crypto::digit_to_signal;

    #[test]
    fn digit_to_signal_returns_correct_for_8() {
        assert_eq!("abcdefg", &digit_to_signal(0b1111111));
    }
}
