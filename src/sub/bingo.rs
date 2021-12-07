mod bingo;
pub mod board;

use crate::sub::bingo::board::BingoBoard;
use crate::Result;

pub fn run_bingo(random_numbers: Vec<u32>, board_inputs: Vec<String>) -> Result<()> {
    let boards_result: Result<Vec<BingoBoard>> = board_inputs
        .iter()
        .map(|board_input| board::BingoBoard::from_str(board_input))
        .collect();
    let mut boards = boards_result?;
    let winners = play_bingo_with_boards(&random_numbers, &mut boards);
    println!(
        "Winning board(s): {}",
        winners
            .iter()
            .map(|i| format!("{}", i + 1))
            .collect::<Vec<String>>()
            .join(", ")
    );
    winners
        .iter()
        .for_each(|i| println!("Score: {}\n{}", boards[*i].get_score(), boards[*i]));
    Ok(())
}

fn play_bingo_with_boards(random_numbers: &Vec<u32>, boards: &mut Vec<BingoBoard>) -> Vec<usize> {
    for n in random_numbers {
        boards.iter_mut().for_each(|board| board.call_number(*n));
        let winning_boards: Vec<usize> = boards
            .iter()
            .enumerate()
            .filter(|(i, board)| board.has_bingo())
            .map(|(i, board)| i)
            .collect();
        if !winning_boards.is_empty() {
            return winning_boards;
        }
    }
    vec![]
}
