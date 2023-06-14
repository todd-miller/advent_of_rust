

pub fn day5() {
    let start: Vec<Vec<char>> = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P'],
    ];

    let (move_, from_, to_) = (1, 2, 1);
    println!("=============== DAY 5 =========================");
    println!("\n\t UNDER CONSTRUCTION");
    move_crates(start, move_, from_, to_);

    println!("===============================================");
    
} 


fn move_crates(crates: Vec<Vec<char>>, n: usize, from: usize, to: usize) -> Vec<Vec<char>> {
    let mut c = crates.to_vec();
    let mut grabbed: Vec<char> = c[from].drain(&crates[from].len()-1..).collect();
    c[to].append(&mut grabbed);

    println!("\t grabbed: {:?}", grabbed);
    println!("\t crates: {:?}", c);
    println!("\t n, from, to: {:?}, {:?}, {:?}", n, from, to);
    
    crates
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let nested_vec: Vec<Vec<char>> = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ]; 

        assert_eq!(nested_vec[0][0], 'Z');
        assert_eq!(nested_vec[1][1], 'C');
        assert_eq!(nested_vec[1][1..=2], ['C', 'D']);
    }

    #[test]
    fn test_moving_crates() {
        let start: Vec<Vec<char>> = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let (move_, from_, to_) = (1, 2, 1);
        let end: Vec<Vec<char>> = vec![
            vec!['Z', 'N', 'D'],
            vec!['M', 'C'],
            vec!['P'],
        ]; 

        assert_eq!(move_crates(start, move_, from_, to_), end );
    }
}

