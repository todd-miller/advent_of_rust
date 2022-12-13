fn part1(l: i8, r: i8) -> u32 {
    let elf = l;
    let usr = r;
    let result = (usr - elf + 1).rem_euclid(3);

    let shape_score = usr + 1;
    let result_score = 3 * result;
    (shape_score + result_score) as u32
}

fn part2(l: i8, r: i8) -> u32 {
    let elf = l;
    let result = r;
    let usr = ( elf - 1 + result).rem_euclid(3);

    let shape_score = usr + 1;
    let result_score = 3* result;
    (shape_score + result_score) as u32
}

fn parser(s: &str) -> (i8, i8) {
    let bytes = s.as_bytes();
    let left = (bytes[0] - b'A') as i8;
    let right = (bytes[2] - b'X') as i8;
    return (left, right);
}

fn calculator(strategy: fn(l: i8, r:i8) -> u32) -> String {
    let input = std::fs::read_to_string("inputs/02/input.txt").unwrap();
    input
        .lines()
        // map every line to the score for that round
        .map(|round| {
            let (left, right) = parser(round);
            strategy(left, right)
        })
        .sum::<u32>()
        .to_string()   
}

pub fn day2() {
    let score_1 = calculator(part1);
    let score_2 = calculator(part2); 
    
    println!("=============== DAY 2 =========================");
    println!("\n \t PART 1 - SCORE: {} \n", score_1);
    println!("\n \t PART 2 - SCORE: {} \n", score_2);
    println!("===============================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("A X"), (0 as i8, 0 as i8));
        assert_eq!(parser("A Y"), (0 as i8, 1 as i8));
        assert_eq!(parser("A Z"), (0 as i8, 2 as i8));
        assert_eq!(parser("B Z"), (1 as i8, 2 as i8));
        assert_eq!(parser("C Z"), (2 as i8, 2 as i8));
    }
    #[test]
    fn test_part1() {
        // ROCK vs. ROCK as a TIE
        assert_eq!(part1(0 as i8, 0 as i8), 4 as u32);

        // ROCK vs. PAPER as a WIN 
        assert_eq!(part1(0 as i8, 1 as i8), 8 as u32);
        
        // ROCK vs. SCISSORS as a LOSS  
        assert_eq!(part1(0 as i8, 2 as i8), 3 as u32);
       
        // SCISSORS vs. ROCK as a WIN
        assert_eq!(part1(2 as i8, 0 as i8), 7 as u32);
    }

    #[test]
    fn test_part2() {
        // ROCK and TIE with ROCK
        assert_eq!(part2(0 as i8, 1 as i8), 4 as u32); 

        // ROCK and WIN with PAPER
        assert_eq!(part2(0 as i8, 2 as i8), 8 as u32); 

        // ROCK and LOSS with SCISSORS 
        assert_eq!(part2(0 as i8, 0 as i8), 3 as u32); 

        // PAPER and WIN with SCISSORS  
        assert_eq!(part2(1 as i8, 2 as i8), 9 as u32); 
    }
}
