use std::num::ParseIntError;
use std::str::FromStr;

pub fn int_lines<'a>(
    input: &'a String,
) -> Box<dyn Iterator<Item = Result<i32, ParseIntError>> + 'a> {
    Box::new(
        input
            .lines()
            .filter(|&slice| slice != "")
            .map(|slice| i32::from_str(slice)),
    )
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
