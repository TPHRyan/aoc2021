use crate::AppParams;

use std::fs::File;
use std::io::{self, BufRead, Read};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn get_file_contents(params: &mut AppParams) -> Result<String, String> {
    let mut file_contents = String::new();
    match params.data_file.read_to_string(&mut file_contents) {
        Ok(_) => Ok(file_contents),
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn int_lines<'a>(
    input: &'a String,
) -> Box<dyn Iterator<Item = Result<usize, ParseIntError>> + 'a> {
    Box::new(
        input
            .lines()
            .filter(|&slice| slice != "")
            .map(|slice| usize::from_str(slice)),
    )
}

pub fn read_data_lines(params: AppParams) -> io::Lines<io::BufReader<File>> {
    let file = params.data_file;
    io::BufReader::new(file).lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_lines_splits_correct_times() {
        let test_str = String::from("28\n1423\n9043\n192");
        let split_str: Vec<Result<usize, ParseIntError>> = int_lines(&test_str).collect();
        assert_eq!(4, split_str.len());
    }
}
