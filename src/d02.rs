use crate::utils::*;
use std::collections::HashMap;
use std::str::SplitWhitespace;

const TIE_VALUE: i32 = 3;
const WIN_VALUE: i32 = 6;
const LOSS_VALUE: i32 = 0;

const TIE: &str = "TIE";
const WIN: &str = "WIN";
const LOSS: &str = "LOSS";

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn p1_round(elf:i32, usr: i32) -> i32 {
    let mut score: i32 = 0;
    if elf == usr {
        score = score + TIE_VALUE;
    } else {
        if (elf == ROCK && usr == PAPER) || (elf == PAPER && usr == SCISSORS) || (elf == SCISSORS && usr == ROCK) {
            score = score + WIN_VALUE;
        } else {
            score = score + LOSS_VALUE;
        }
    }
    score = score + usr;
    return score;
}

fn p2_round(elf:i32, usr: &str) -> i32 {
    let mut score: i32 = 0;
    
    if usr == TIE {
        score = score + TIE_VALUE;
        score = score + elf;
    } else if usr == WIN {
        score = score + WIN_VALUE;
        if elf == PAPER {
            score = score + SCISSORS;
        } else if elf == ROCK {
            score = score + PAPER;
        } else if elf == SCISSORS {
            score = score + ROCK;
        }
    } else if usr == LOSS {
        score = score + LOSS_VALUE;
         if elf == PAPER {
            score = score + ROCK;
        } else if elf == ROCK {
            score = score + SCISSORS;
        } else if elf == SCISSORS {
            score = score + PAPER;
        }
    }
    return score;
}
 
fn p1_moves(round: &str) -> (i32, i32) {
    let map: HashMap<&str, i32> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
        ("X", ROCK),
        ("Y", PAPER),
        ("Z", SCISSORS),
    ]);
    let mut moves: SplitWhitespace = round.split_whitespace();
    let (elf_code, user_code) = (moves.next().unwrap(), moves.next().unwrap() );  
    return (map[elf_code], map[user_code]);
}

fn p2_moves(round: &str) -> (i32, &str) {
    let map1: HashMap<&str, i32> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
    ]);
    let map2: HashMap<&str, &str> = HashMap::from([
        ("X", LOSS),
        ("Y", TIE),
        ("Z", WIN),

    ]); 
    let mut moves: SplitWhitespace = round.split_whitespace();
    let (elf_code, user_code) = (moves.next().unwrap(), moves.next().unwrap() );  
    return (map1[elf_code], map2[user_code]);
}


fn p1_calc(games: &Vec<String>) -> i32 {
    let mut score: i32 = 0;
    for game in games {
        let (elf, user) = p1_moves(&game);
        score = score + p1_round(elf, user);
    }
    score
}

fn p2_calc(games: &Vec<String>) -> i32 {
    let mut score: i32 = 0;
    for game in games {
        let (elf, user) = p2_moves(&game);
        score = score + p2_round(elf, user);
    }
    score
}

pub fn day2() {
    let games = lines_from_file("inputs/02/input.txt").expect("could not read file");
    let score_1 = p1_calc(&games); 
    let score_2 = p2_calc(&games); 
    
    println!("=============== DAY 2 =========================");
    println!("\n \t PART 1 - SCORE: {} \n", score_1);
    println!("\n \t PART 2 - SCORE: {} \n", score_2);
    println!("===============================================");
}

