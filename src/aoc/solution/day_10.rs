use crate::sub::navigation::{
    calculate_completion_score, parse_chunks_for_lines, ChunkStyle, ParseError,
};
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

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let line_results = parse_chunks_for_lines(&challenge_data);
    let unexpected_eol_stacks: Vec<Vec<char>> = line_results
        .into_iter()
        .filter_map(|line| match line {
            Err(err) => match err {
                ParseError::EOL(stack) => Some(stack),
                _ => None,
            },
            _ => None,
        })
        .collect();
    let mut completion_scores: Vec<u64> = unexpected_eol_stacks
        .into_iter()
        .map(|mut stack| calculate_completion_score(&mut stack))
        .collect();
    completion_scores.sort();
    let middle_index = (completion_scores.len() - 1) / 2;
    println!(
        "Middle completion score: {}",
        completion_scores[middle_index]
    );
    Ok(())
}
