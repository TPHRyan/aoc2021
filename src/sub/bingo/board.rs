use super::bingo::{Bingo, BingoCallResult, BingoTrigger};
use crate::Result;
use serde_scan;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct BingoBoard {
    numbers: HashMap<u32, [usize; 2]>,
    called_numbers: HashMap<u32, bool>,
    filled_cols: [Vec<u32>; 5],
    filled_rows: [Vec<u32>; 5],
    bingo: Option<Bingo>,
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
            called_numbers: HashMap::new(),
            filled_cols: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            filled_rows: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            bingo: None,
        })
    }

    pub fn call_number(&mut self, n: u32) -> BingoCallResult {
        if self.numbers.contains_key(&n) && !self.called_numbers.contains_key(&n) {
            self.called_numbers.insert(n, true);
            let [x, y] = self.numbers[&n];
            self.filled_cols[x].push(n);
            self.filled_rows[y].push(n);

            return if self.check_bingo(x, y, n) {
                BingoCallResult::BINGO
            } else {
                BingoCallResult::CROSS
            };
        }
        BingoCallResult::NONE
    }

    fn check_bingo(&mut self, col: usize, row: usize, called_number: u32) -> bool {
        return if self.bingo.is_some() {
            false
        } else if self.filled_cols[col].len() >= 5 {
            self.bingo = Some(Bingo {
                final_number: called_number,
                triggered_by: BingoTrigger::COLUMN(col),
            });
            true
        } else if self.filled_rows[row].len() >= 5 {
            self.bingo = Some(Bingo {
                final_number: called_number,
                triggered_by: BingoTrigger::ROW(row),
            });
            true
        } else {
            false
        };
    }

    pub fn get_score(&self) -> u32 {
        match &self.bingo {
            Some(bingo) => {
                let unmarked_numbers_total = self.total_unmarked_numbers();
                bingo.final_number * unmarked_numbers_total
            }
            None => 0,
        }
    }

    fn total_unmarked_numbers(&self) -> u32 {
        self.numbers
            .keys()
            .filter(|&n| !self.called_numbers.contains_key(n))
            .fold(0, |acc, n| acc + n)
    }

    #[cfg(test)]
    pub fn has_bingo(&self) -> bool {
        self.bingo.is_some()
    }

    #[cfg(test)]
    pub fn longest_len(&self) -> usize {
        let mut longest: usize = 0;
        for col in self.filled_cols.iter() {
            longest = std::cmp::max(longest, col.len());
        }
        for row in self.filled_rows.iter() {
            longest = std::cmp::max(longest, row.len());
        }
        longest
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_data: [[u32; 5]; 5] = [[0; 5]; 5];
        for k in self.numbers.keys() {
            let [x, y] = self.numbers[k];
            board_data[y][x] = *k;
        }
        let bingo_col: usize =
            self.bingo
                .as_ref()
                .map_or(usize::MAX, |bingo| match bingo.triggered_by {
                    BingoTrigger::COLUMN(col) => col,
                    _ => usize::MAX,
                });
        let bingo_row: usize =
            self.bingo
                .as_ref()
                .map_or(usize::MAX, |bingo| match bingo.triggered_by {
                    BingoTrigger::ROW(row) => row,
                    _ => usize::MAX,
                });
        let mut board_str = String::new();
        for (row, row_arr) in board_data.iter().enumerate() {
            for (col, val) in row_arr.iter().enumerate() {
                let raw_val = format!("{}", val);
                let val_formatted = if self.called_numbers.contains_key(&val) {
                    if col == bingo_col || row == bingo_row {
                        format!("\x1B[9m{}\x1B[29m", raw_val)
                    } else {
                        format!("\x1B[4m{}\x1B[24m", raw_val)
                    }
                } else {
                    format!("{}", raw_val)
                };
                let val_str = format!(
                    "{}{}",
                    if raw_val.len() < 3 {
                        " ".repeat(3 - raw_val.len())
                    } else {
                        String::from("")
                    },
                    val_formatted,
                );
                board_str = format!("{}{}", board_str, val_str);
            }
            board_str += "\n";
        }
        write!(f, "{}", board_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOARD: &str = "
            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
        ";

    #[test]
    fn can_parse_bingo_board() {
        let board = get_test_board();
        let board_str = format!("{}", board);
        assert_eq!(
            "2213171108223424219141676103185112201519",
            board_str.replace(" ", "").replace("\n", "")
        );
    }

    #[test]
    fn calling_owned_numbers_increases_line() {
        let mut board = get_test_board();
        assert_eq!(0, board.longest_len());
        board.call_number(21);
        assert_eq!(1, board.longest_len());
    }

    #[test]
    fn calling_unowned_numbers_does_not_increase_line() {
        let mut board = get_test_board();
        assert_eq!(0, board.longest_len());
        board.call_number(30);
        assert_eq!(0, board.longest_len());
    }

    #[test]
    fn calling_same_number_twice_does_not_increase_line() {
        let mut board = get_test_board();
        assert_eq!(0, board.longest_len());
        board.call_number(3);
        assert_eq!(1, board.longest_len());
        board.call_number(3);
        assert_eq!(1, board.longest_len());
    }

    #[test]
    fn calling_column_of_numbers_gives_bingo() {
        let mut board = get_test_board();
        assert_eq!(0, board.longest_len());
        assert!(!board.has_bingo());
        board.call_number(3);
        assert_eq!(1, board.longest_len());
        assert!(!board.has_bingo());
        board.call_number(14);
        assert_eq!(2, board.longest_len());
        assert!(!board.has_bingo());
        board.call_number(17);
        assert_eq!(3, board.longest_len());
        assert!(!board.has_bingo());
        board.call_number(20);
        assert_eq!(4, board.longest_len());
        assert!(!board.has_bingo());
        board.call_number(23);
        assert_eq!(5, board.longest_len());
        assert!(board.has_bingo());
    }

    fn get_test_board() -> BingoBoard {
        let board_result = BingoBoard::from_str(TEST_BOARD);
        assert!(board_result.is_ok());
        board_result.unwrap()
    }
}
