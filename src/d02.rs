use crate::utils::*;
use std::collections::HashMap;

const TIE: i32 = 3;
const WIN: i32 = 6;
const LOSS: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;


fn evaluate_round(elf:i32, usr: i32) -> i32 {
    let mut score: i32 = 0;
    if elf == usr {
        score = score + TIE;
    } else {
        if (elf == ROCK && usr == PAPER) || (elf == PAPER && usr == SCISSORS) || (elf == SCISSORS && usr == ROCK) {
            score = score + WIN;
        } else {
            score = score + LOSS;
        }
    }
    score = score + usr;
    return score;
}

fn parse_moves(round: &str) -> (i32, i32) {
    let map: HashMap<&str, i32> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
        ("X", ROCK),
        ("Y", PAPER),
        ("Z", SCISSORS),
    ]);
    let mut moves: std::str::SplitWhitespace = round.split_whitespace();
    let (elf_code, user_code) = (moves.next().unwrap(), moves.next().unwrap() );  
    return (map[elf_code], map[user_code]);
}

fn calculator(games: Vec<String>) -> i32 {
    let mut score: i32 = 0;
    for game in games {
        let (elf, user) = parse_moves(&game);
        score = score + evaluate_round(elf, user);
    }
    score
}

pub fn day2() {
    let games = lines_from_file("inputs/02/input.txt").expect("could not read file");
    let score = calculator(games); 
    
    println!("=============== DAY 2 =========================");
    println!("\n \t PART 1 - SCORE: {} \n", score);
    println!("\n \t PART 2 - TODO!");
    println!("===============================================");
}

