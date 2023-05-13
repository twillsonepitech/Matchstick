mod game;
mod parse;
mod setup;

use setup::setup_sstc;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <number-of-matches> <max-matches-per-turn>", args[0]);
        process::exit(84);
    }

    let num_matches: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number of matches");
            process::exit(84);
        }
    };

    let max_matches_per_turn: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid max matches per turn");
            process::exit(84);
        }
    };

    if num_matches <= 1 || num_matches >= 100 {
        println!("Error on parameter: <number-of-matches>. (must be 1 < n < 100)");
        process::exit(84);
    }

    if max_matches_per_turn < 1 {
        println!("Error on parameter: <max-matches-per-turn>. (must be > 0)");
        process::exit(84);
    }

    let result : u8 = setup_sstc(num_matches, max_matches_per_turn);

    process::exit(result.into());
}

