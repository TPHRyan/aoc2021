mod bingo;
pub mod board;

use crate::Result;

pub fn run_bingo(random_numbers: Vec<u32>, board_inputs: Vec<String>) -> Result<()> {
    println!("Random numbers: {:?}", random_numbers);
    println!("Boards:");
    for board_input in board_inputs {
        let parsed_board = board::BingoBoard::from_str(&board_input)?;
        println!("{}", parsed_board)
    }
    Ok(())
}
