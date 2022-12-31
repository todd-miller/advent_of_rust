use std::{collections::HashMap, usize};

const INPUT: &str = "inputs/03/input.txt";


// TODO - come back to this and try to use the iter-map style?

pub fn day3() {
    let input: String = std::fs::read_to_string(INPUT).unwrap();
    let map: HashMap<char, i32> = create_map();    
    let p1 = part1(&input, &map); 
    let p2 = part2(&input, &map); 
  
    println!("=============== DAY 3 =========================");
    println!("\n\t PART 1 - PRIORITY: {:?}\n", p1);
    println!("\n\t PART 2 - PRIORITY: {:?}\n", p2);
    println!("===============================================");
}

fn create_map() -> HashMap<char, i32> { 
    let mut map:HashMap<char, i32> = HashMap::new();
    let lower = (b'a'..=b'z').map(|c| c as char); 
    let upper = (b'A'..=b'Z').map(|c| c as char); 
    lower
        .chain(upper)
        .zip(1..=52)
        .for_each(|(c, i)| {
           map.insert(c, i);
        });
    map
}

fn part1(input: &str, map: &HashMap<char, i32>) -> i32 {
    let mut priority = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for item in left.chars() {
            if right.contains(item) {
                priority += *map.get(&item).unwrap();
                break;
            }
        }
    }
    priority
}

fn part2(input: &str, map: &HashMap<char, i32>) -> i32 {
    let mut badge_priority = 0;
    for (index, _ ) in input.lines().enumerate().step_by(3) {
        let badge = get_badge(index, input);
        badge_priority += map.get(&badge).unwrap();
    } 
    badge_priority 
}

fn get_badge(index: usize, input: &str) -> char {
    let l1 = input.lines().nth(index).unwrap(); 
    let l2 = input.lines().nth(index+1).unwrap(); 
    let l3 = input.lines().nth(index+2).unwrap(); 
    
    let mut badge: char = '_';
    for c in l1.chars() {
        if l2.contains(c) && l3.contains(c) {
            badge = c;
            break;
        }
    } 
    badge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let input: String = std::fs::read_to_string("inputs/03/test.txt").unwrap();
        assert_eq!(part1(&input, &create_map()), 157);
    }
}

