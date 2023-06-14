
pub fn day4() {
    let (part1, part2) = main();
    println!("=============== DAY 4 =========================");
    println!("\n\t PART 1 - SUB-SETS: {:?}", part1);
    println!("\n\t PART 2 - OVERLAPS: {:?}", part2);
    println!("===============================================");
} 

fn main() -> (i32, i32) {
    let input = std::fs::read_to_string("inputs/04/input.txt").unwrap();
    let mut part1: i32 = 0; 
    let mut part2: i32 = 0; 

    for line in input.lines() {
        if contains_range(split_ranges(line)) {
            part1 += 1;
        }
        if contains_edge(split_ranges(line)) {
            part2 += 1;
        }
    }

    (part1, part2)
}


fn split_ranges(s: &str) -> (i32, i32, i32, i32) {
    let comma = ","; 
    let dash = "-"; 
    let (a, b) = s.split_once(comma).unwrap();
    let (ab, at) = a.split_once(dash).unwrap(); 
    let (bb, bt) = b.split_once(dash).unwrap(); 

    (
        ab.parse::<i32>().unwrap(), 
        at.parse::<i32>().unwrap(),
        bb.parse::<i32>().unwrap(),
        bt.parse::<i32>().unwrap(),
    )
}

fn contains_range((ab, at, bb, bt): (i32, i32, i32, i32)) -> bool {
    (ab <= bb && at >= bt) || ( bb <= ab && bt >= at)
}

fn contains_edge((ab, at, bb, bt): (i32, i32, i32, i32)) -> bool {
    let a_range = ab..=at;
    let b_range = bb..=bt;
    a_range.contains(&bb) || a_range.contains(&bt) || b_range.contains(&ab) || b_range.contains(&at)
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_ranges() {
        assert_eq!(split_ranges("2-4,6-8"), (2, 4, 6, 8));
        assert_eq!(split_ranges("22-44,66-88"), (22, 44, 66, 88));
    }

    #[test]
    fn test_contains_range() {
        assert!(!contains_range((2, 4, 6, 8)));
        assert!(!contains_range((2, 3, 4, 5)));
        assert!(contains_range((1, 99, 2, 98)));
    }

    #[test]
    fn test_contains_edge() {
        assert!(!contains_edge((2, 4, 6, 8)));
        assert!(!contains_edge((2, 3, 4, 5)));
        assert!(contains_edge((1, 99, 2, 98)));
        assert!(contains_edge((1, 2, 2, 4)));
        assert!(contains_edge((5, 7, 7, 9)));
        assert!(contains_edge((6, 6, 4, 6)));
        assert!(contains_edge((2, 6, 4, 8)));
        assert!(contains_edge((2, 8, 3, 7)));
    }
   
}


