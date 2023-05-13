use crate::game::execute_game;
use rand::prelude::*;

const AI: u8 = 1;
// const NONE: u8 = 0;
const PLAYER: u8 = 2;

pub struct Board {
    pub _board: Vec<Vec<char>>,
    pub _num_matches : usize,
}

impl Board {
    pub fn print_board(&self) {
        for _ in 0..self._num_matches * 2 - 1 + 2 {
            print!("*");
        }
        println!();

        for i in 0..self._num_matches {
            println!("*{}*", self._board[self._num_matches - i - 1].iter().collect::<String>());
        }

        for _ in 0..self._num_matches * 2 - 1 + 2 {
            print!("*");
        }
        println!();
    }

    pub fn line_count_pipes(&self, line: usize) -> usize {
        let mut counter : usize = 0;

        for i in 0..self._num_matches * 2 - 1 {
            if self._board[self._num_matches - line][i] == '|' {
                counter += 1;
            }
        }
        return counter;
    }

    pub fn board_is_empty(&self) -> bool {
        let mut counter : usize = 0;

        for i in 0..self._num_matches {
            for j in 0..self._num_matches * 2 - 1 {
                if self._board[i][j] == '|' {
                    counter += 1;
                }
            }
        }
        return if counter == 0 { true } else { false };
    }

    pub fn change_board(&mut self, line: usize, matches: usize) {
        let mut counter : usize = 0;

        for i in (0..self._num_matches * 2 - 1).rev() {
            if counter == matches {
                break;
            }
            if self._board[self._num_matches - line][i] == '|' {
                self._board[self._num_matches - line][i] = ' ';
                counter += 1;
            }
        }
    }

    pub fn catch_valid_line(&self) -> usize {
        let mut lines: Vec<usize> = Vec::new();
    
        for i in 0..self._num_matches {
            if self.line_count_pipes(i + 1) != 0 {
                lines.push(i + 1);
            }
        }
        let random_index = rand::thread_rng().gen_range(0..lines.len());
        return lines[random_index];
    }

    pub fn catch_valid_matches(&self, max_matches_per_turn: usize, line: usize) -> usize {
        fn min(a: usize, b: usize) -> usize {
            return if a < b { a } else { b };
        }
        let num_of_pipes : usize = self.line_count_pipes(line);
        if num_of_pipes == 1 {
            return num_of_pipes;
        }
        let minimum : usize = min(max_matches_per_turn, num_of_pipes);
        let random_matches : usize = rand::thread_rng().gen_range(1..minimum);
        return random_matches;
    }
}

pub fn setup_sstc(num_matches : usize, max_matches_per_turn : usize) -> u8 {
    let mut array: Vec<Vec<char>> = Vec::with_capacity(num_matches + 1);
    let mut space = 0;

    for _ in 0..num_matches {
        let mut row = Vec::with_capacity(num_matches * 2);
        for _ in 0..space {
            row.push(' ');
        }
        for _ in space..num_matches * 2 - 1 - space {
            row.push('|');
        }
        for _ in num_matches * 2 - 1 - space..num_matches * 2 - 1 {
            row.push(' ');
        }
        row.push('\0');
        array.push(row);
        space += 1;
    }
    array.push(vec![]);

    let mut board = Board { _board: array, _num_matches: num_matches };
    let mut result: u8;

    board.print_board();
    loop {
        result = execute_game(&mut board, max_matches_per_turn);
        if result == AI || result == PLAYER {
            break;
        }
    }
    return result;
}

