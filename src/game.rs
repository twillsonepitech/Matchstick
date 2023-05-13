use crate::setup::Board;
use crate::parse::read_one_line;
use crate::parse::parse_readed_line;

const ERROR: i32 = -1;
const ZERO: usize = 0;

const AI: u8 = 1;
const NONE: u8 = 0;
const PLAYER: u8 = 2;

#[derive(PartialEq)]
pub enum Turn {
    Line,
    Matches,
}

fn execute_player(board: &mut Board, max_matches_per_turn: usize) {
    println!("\nYour turn:");
    loop {
        let readed_line : String = read_one_line("Line: ".to_string());
        let line : i32 = parse_readed_line(board, max_matches_per_turn, readed_line, ZERO, Turn::Line);
        if line == ERROR {
            continue;
        }
        let readed_matches : String = read_one_line("Matches: ".to_string());
        let matches : i32 = parse_readed_line(board, max_matches_per_turn, readed_matches, line.try_into().unwrap(), Turn::Matches);
        if matches == ERROR {
            continue;
        }
        println!("Player removed {} match(es) from line {}", matches, line);
        board.change_board(line.try_into().unwrap(), matches.try_into().unwrap());
        break;
    }
}

fn execute_ai(board: &mut Board, max_matches_per_turn: usize) {
    println!("\nAI's turn...");
    let line : usize = board.catch_valid_line();
    let matches : usize = board.catch_valid_matches(max_matches_per_turn, line);
    println!("AI removed {} match(es) from line {}", matches, line);
    board.change_board(line, matches);
}

pub fn execute_game(board: &mut Board, max_matches_per_turn: usize) -> u8 {
    execute_player(board, max_matches_per_turn);
    board.print_board();
    if board.board_is_empty() == true {
        println!("You lost, too bad...");
        return PLAYER;
    }
    execute_ai(board, max_matches_per_turn);
    board.print_board();
    if board.board_is_empty() == true {
        println!("I lost... snif... but I'll get you next time!!");
        return AI;
    }
    return NONE;
}

