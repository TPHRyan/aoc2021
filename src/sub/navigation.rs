mod chunk;

pub use chunk::{parse_chunks_for_line, Chunk, ChunkStyle, ParseError, ParseResult};

pub fn parse_chunks_for_lines(input_lines: &str) -> Vec<ParseResult> {
    input_lines
        .lines()
        .map(|line| parse_chunks_for_line(line))
        .collect()
}

pub fn calculate_completion_score(stack: &mut Vec<char>) -> u64 {
    let mut score: u64 = 0;
    while let Some(c) = stack.pop() {
        score *= 5;
        score += score_for_char(c);
    }
    score
}

fn score_for_char(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
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

    #[test]
    fn calculate_completion_score_works_for_example() {
        let test_str = "<{([";
        let mut stack: Vec<char> = test_str.chars().collect();
        let expected_score = 294;
        let actual_score = calculate_completion_score(&mut stack);
        assert_eq!(expected_score, actual_score);
    }
}
