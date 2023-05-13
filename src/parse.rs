use crate::setup::Board;
use crate::game::Turn;
use std::io::{self, BufRead, Write};
use std::process;

const ERROR: i32 = -1;

pub fn read_one_line(syntax: String) -> String {
    print!("{}", syntax);
    _ = io::stdout().flush();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let result = stdin.lock().read_line(&mut buffer);
    match result {
        Err(error) => {
            eprintln!("Erreur de lecture: {}", error);
            process::exit(84);
        }
        Ok(0) => { process::exit(0); }
        Ok(_) => { buffer.pop(); }
    }
    return buffer;
}

fn parse_line(num_matches: usize, ret: i32) -> i32 {
    if ret <= 0 || ret > num_matches.try_into().unwrap() {
        println!("Error: this line is out of range");
        return ERROR;
    }
    return ret;
}

fn parse_matches(num_of_pipes: usize, max_matches_per_turn: usize, ret: i32) -> i32 {
    if ret <= 0 {
        println!("Error: you have to remove at least one match");
        return ERROR;
    }
    if ret > max_matches_per_turn.try_into().unwrap() {
        println!("Error: you cannot remove more than {} matches per turn", max_matches_per_turn);
        return ERROR;
    }
    if ret > num_of_pipes.try_into().unwrap() {
        println!("Error: not enough matches on this line");
        return ERROR;
    }
    return ret;
}

pub fn parse_readed_line(board: &Board, max_matches_per_turn: usize, buffer: String, line: usize, turn: Turn) -> i32 {
    if let Ok(num) = buffer.parse::<i32>() {
        return if turn == Turn::Line { parse_line(board._num_matches, num) } else { parse_matches(board.line_count_pipes(line), max_matches_per_turn, num) };
    } else {
        println!("Error: invalid input (positive number expected)");
        return ERROR;
    }
}

