mod chunk;

pub use chunk::{parse_chunks_for_line, Chunk, ChunkStyle, ParseError, ParseResult};

pub fn parse_chunks_for_lines(input_lines: &str) -> Vec<ParseResult> {
    input_lines
        .lines()
        .map(|line| parse_chunks_for_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::chunk::{ChunkStyle, ParseError};
    use super::*;

    const TEST_STR: &str = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn parse_chunks_for_lines_works_for_example_data() {
        let results = parse_chunks_for_lines(TEST_STR);
        let expected_style_errors = vec![
            ParseError::EncounteredStyle(ChunkStyle::Curly),
            ParseError::EncounteredStyle(ChunkStyle::Paren),
            ParseError::EncounteredStyle(ChunkStyle::Square),
            ParseError::EncounteredStyle(ChunkStyle::Paren),
            ParseError::EncounteredStyle(ChunkStyle::Angle),
        ];
        let encountered_style_errors: Vec<ParseError> = results
            .iter()
            .filter_map(|result| match result {
                Err(parse_error) => match parse_error {
                    ParseError::EncounteredStyle(style) => {
                        Some(ParseError::EncounteredStyle(*style))
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();
        for (expected_error, actual_error) in expected_style_errors
            .iter()
            .zip(encountered_style_errors.iter())
        {
            assert_eq!(expected_error, actual_error);
        }
    }
}
