use std::error::Error as ErrorTrait;

use super::{Chunk, ChunkStyle};

#[derive(Debug)]
pub enum ParseError {
    EncounteredStyle(ChunkStyle),
    EOL(Vec<char>),
    Error(Box<dyn ErrorTrait>),
    UnrecognizedChar,
}

impl From<Box<dyn ErrorTrait>> for ParseError {
    fn from(err: Box<dyn ErrorTrait>) -> Self {
        ParseError::Error(err)
    }
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::EncounteredStyle(style) => match other {
                Self::EncounteredStyle(other_style) => style == other_style,
                _ => false,
            },
            Self::EOL(_) => match other {
                Self::EOL(_) => true,
                _ => false,
            },
            Self::UnrecognizedChar => match other {
                Self::UnrecognizedChar => true,
                _ => false,
            },
            Self::Error(_) => false,
        }
    }
}

pub type ParseResult = Result<Vec<Chunk>, ParseError>;

pub fn parse_chunks_for_line(input: &str) -> ParseResult {
    let mut chars = input.chars();
    let mut stack: Vec<char> = vec![];
    let mut chunks: Vec<Box<Chunk>> = vec![];
    while let Some(c) = chars.next() {
        match c {
            '<' | '{' | '(' | '[' => {
                let mut nested_chunks = &mut chunks;
                for _ in 0..stack.len() {
                    nested_chunks = &mut nested_chunks
                        .last_mut()
                        .expect("Empty chunk stack while stack was not empty!")
                        .chunks;
                }
                nested_chunks.push(Box::new(Chunk::new(chunk_style_for_char(c)?)));
                stack.push(c);
            }
            '>' | '}' | ')' | ']' => {
                if let Some(start) = stack.pop() {
                    if !chunk_style_matches(start, c) {
                        return Err(ParseError::EncounteredStyle(chunk_style_for_char(c)?));
                    }
                } else {
                    return Err(ParseError::EncounteredStyle(chunk_style_for_char(c)?));
                }
            }
            _ => return Err(ParseError::UnrecognizedChar),
        }
    }
    if stack.is_empty() {
        Ok(chunks.into_iter().map(|chunk_box| *chunk_box).collect())
    } else {
        Err(ParseError::EOL(stack))
    }
}

fn chunk_style_for_char(c: char) -> Result<ChunkStyle, ParseError> {
    match c {
        '<' | '>' => Ok(ChunkStyle::Angle),
        '{' | '}' => Ok(ChunkStyle::Curly),
        '(' | ')' => Ok(ChunkStyle::Paren),
        '[' | ']' => Ok(ChunkStyle::Square),
        _ => Err(ParseError::UnrecognizedChar),
    }
}

fn chunk_style_matches(start: char, end: char) -> bool {
    match (start, end) {
        ('<', '>') => true,
        ('{', '}') => true,
        ('(', ')') => true,
        ('[', ']') => true,
        _ => false,
    }
}
