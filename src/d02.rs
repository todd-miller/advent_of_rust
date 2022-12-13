use crate::utils::*;
use std::collections::HashMap;

const TIE: i32 = 3;
const WIN: i32 = 6;
const LOSS: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn calculator(games: Vec<String>) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
        ("X", ROCK),
        ("Y", PAPER),
        ("Z", SCISSORS),
    ]);
     
    let mut score: i32 = 0;
    for game in games {
        let mut round: std::str::SplitWhitespace = game.split_whitespace();
        let (elf_code, user_code) = (round.next().unwrap(), round.next().unwrap() );  
        let elf = map[elf_code];
        let usr = map[user_code];

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


// maybe i should write a test?
// i know i need to store some stuff
// X - rock
// Y - paper
// Z - scissors 
