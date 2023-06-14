use crate::utils::*;

fn calculator(content: Vec<String>) -> Vec<i32> {
    let mut totals = vec![];
    let mut count:i32 = 0;
 
    for line in content {
        if !line.is_empty() {
            let num:i32 = line.parse().unwrap();
            count += num; 
        } else {
            totals.push(count);
            count = 0;
        }
    }
    totals.sort();
    totals.reverse();
    totals
}

pub fn day1() {
    let lines = lines_from_file("inputs/01/input.txt").expect("could not read lines");
    let elf_totals = calculator(lines);
    let top_elves = elf_totals.to_vec().drain(..3).sum::<i32>(); 
    let top_elf = elf_totals.iter().max().unwrap();

    println!("=============== DAY 1 =========================");
    println!("\n\tPART 1 - CALORIES: {}\n", top_elf);
    println!("\n\tPART 2 - CALORIES: {}\n", top_elves); 
    println!("===============================================");
}
