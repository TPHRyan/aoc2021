use crate::{Error, Result};
use std::fmt::{Display, Formatter};

pub struct SevenSegments {
    value: u8,
}

impl SevenSegments {
    pub fn from(value: u8) -> SevenSegments {
        SevenSegments { value }
    }

    pub fn from_str(s: &str) -> Result<SevenSegments> {
        Ok(SevenSegments {
            value: word_to_segments(s)?,
        })
    }

    pub fn has_unique_segment_count(&self) -> bool {
        match self.num_segments_on() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }

    pub fn num_segments_on(&self) -> u8 {
        let mut total = 0;
        for n in 0..8 {
            if self.value & (1 << n) > 0 {
                total += 1;
            }
        }
        total
    }
}

impl Display for SevenSegments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", segments_to_word(self.value))
    }
}

pub struct OutputDisplay {
    pub segments: Vec<SevenSegments>,
}

impl OutputDisplay {
    pub fn from(segments: Vec<SevenSegments>) -> OutputDisplay {
        OutputDisplay { segments }
    }

    pub fn from_str(s: &str) -> Result<OutputDisplay> {
        let segments: Vec<SevenSegments> = s
            .split_whitespace()
            .map(|word| SevenSegments::from_str(word))
            .collect::<Result<Vec<SevenSegments>>>()?;
        Ok(OutputDisplay { segments })
    }

    pub fn unique_digits(&self) -> Vec<&SevenSegments> {
        self.segments
            .iter()
            .filter(|&seg| seg.has_unique_segment_count())
            .collect()
    }
}

impl Display for OutputDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.segments
                .iter()
                .map(|segments| format!("{}", segments))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

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

fn segments_to_word(segments: u8) -> String {
    let mut word = String::new();
    for n in 0..8 {
        let masked_bit = segments & (1 << n);
        if masked_bit > 0 {
            word = format!("{}{}", bit_to_segment_letter(masked_bit), word);
        }
    }
    word
}

fn word_to_segments(word: &str) -> Result<u8> {
    let bits: Vec<u8> = word
        .chars()
        .map(|c| segment_letter_to_bit(c))
        .collect::<Result<Vec<u8>>>()?;
    Ok(bits.iter().fold(0, |acc, bit| acc | bit))
}
