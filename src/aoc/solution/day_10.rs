use crate::sub::navigation::{parse_chunks_for_lines, ChunkStyle, ParseError};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let line_results = parse_chunks_for_lines(&challenge_data);
    let unexpected_styles: Vec<ChunkStyle> = line_results
        .into_iter()
        .filter_map(|line| match line {
            Err(err) => match err {
                ParseError::EncounteredStyle(style) => Some(style),
                _ => None,
            },
            _ => None,
        })
        .collect();
    let error_score = unexpected_styles.iter().fold(0, |acc, style| {
        acc + match style {
            ChunkStyle::Paren => 3,
            ChunkStyle::Square => 57,
            ChunkStyle::Curly => 1197,
            ChunkStyle::Angle => 25137,
        }
    });
    println!("Error score: {}", error_score);
    Ok(())
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}
