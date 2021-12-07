mod bingo;
pub mod board;

pub use crate::sub::bingo::board::BingoBoard;
use crate::Result;

pub fn find_winning_bingo_boards(
    random_numbers: Vec<u32>,
    mut boards: Vec<BingoBoard>,
) -> Result<()> {
    let winners = play_bingo_with_winning_boards(&random_numbers, &mut boards);
    display_index_header("Winning board(s)", &winners);
    display_boards_with_score_by_index(&winners, &boards);
    Ok(())
}

pub fn find_losing_bingo_boards(
    random_numbers: Vec<u32>,
    mut boards: Vec<BingoBoard>,
) -> Result<()> {
    let losers = play_bingo_with_losing_boards(&random_numbers, &mut boards);
    display_index_header("Losing board(s)", &losers);
    display_boards_with_score_by_index(&losers, &boards);
    Ok(())
}

fn display_index_header(caption: &str, indices: &Vec<usize>) {
    println!(
        "{}: {}",
        caption,
        indices
            .iter()
            .map(|i| format!("{}", i + 1))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

fn display_boards_with_score_by_index(indices: &Vec<usize>, boards: &Vec<BingoBoard>) {
    indices
        .iter()
        .for_each(|i| println!("Score: {}\n{}", boards[*i].get_score(), boards[*i]));
}

fn play_bingo_with_winning_boards(
    random_numbers: &Vec<u32>,
    boards: &mut Vec<BingoBoard>,
) -> Vec<usize> {
    for n in random_numbers {
        let winning_boards = call_returning_winning_boards(*n, boards);
        if !winning_boards.is_empty() {
            return winning_boards;
        }
    }
    vec![]
}

fn play_bingo_with_losing_boards(
    random_numbers: &Vec<u32>,
    boards: &mut Vec<BingoBoard>,
) -> Vec<usize> {
    let mut winning_boards: Vec<usize> = Vec::new();
    for n in random_numbers {
        winning_boards.append(&mut call_returning_winning_boards(*n, boards));
        if winning_boards.len() >= boards.len() {
            return vec![*winning_boards.last().unwrap()];
        }
    }
    vec![]
}

fn call_returning_winning_boards(n: u32, boards: &mut Vec<BingoBoard>) -> Vec<usize> {
    boards
        .iter_mut()
        .enumerate()
        .filter_map(|(i, board)| match board.call_number(n) {
            bingo::BingoCallResult::BINGO => Some(i),
            _ => None,
        })
        .collect()
}
