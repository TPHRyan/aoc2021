use crate::Result;
use serde_scan;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct BingoBoard {
    numbers: HashMap<u32, [usize; 2]>,
    filled_cols: Vec<Vec<u32>>,
    filled_rows: Vec<Vec<u32>>,
}

impl BingoBoard {
    pub fn from_str(s: &str) -> Result<BingoBoard> {
        let board_data: [[u32; 5]; 5] = serde_scan::from_str(s)?;
        let mut board_numbers: HashMap<u32, [usize; 2]> = HashMap::new();
        for y in 0..5 {
            for x in 0..5 {
                let number_at_pos: u32 = board_data[y][x];
                board_numbers.insert(number_at_pos, [x, y]);
            }
        }
        Ok(BingoBoard {
            numbers: board_numbers,
            filled_cols: Vec::new(),
            filled_rows: Vec::new(),
        })
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_data: [[u32; 5]; 5] = [[0; 5]; 5];
        for k in self.numbers.keys() {
            let [x, y] = self.numbers[k];
            board_data[y][x] = *k;
        }
        let mut board_str = String::new();
        for row in board_data {
            for col in row {
                board_str = format!("{}{:>3}", board_str, col);
            }
            board_str += "\n";
        }
        write!(f, "{}", board_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_bingo_board() {
        let test_board = "
            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
        ";
        let board_result = BingoBoard::from_str(test_board);
        assert!(board_result.is_ok());
        let board = board_result.unwrap();
        let board_str = format!("{}", board);
        assert_eq!(
            "2213171108223424219141676103185112201519",
            board_str.replace(" ", "").replace("\n", "")
        );
    }
}
