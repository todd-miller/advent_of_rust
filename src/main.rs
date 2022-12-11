use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn calculator(content: Vec<String>) -> Vec<i32> {
    let mut totals = vec![];
    let mut count:i32 = 0;
 
    for line in content {
        if !line.is_empty() {
            let num:i32 = line.parse().unwrap();
            count = count + num; 
        } else {
            totals.push(count);
            count = 0;
        }
    }
    totals.sort();
    totals.reverse();
    totals
}

fn main() {
    let lines = lines_from_file("inputs/01/input.txt").expect("could not read lines");
    let mut elf_totals = calculator(lines);
    let top_elves = elf_totals.drain(..3); 
    // need to sum the first three items in the vec elf_totals
    println!("{}", top_elves.sum::<i32>())
    
}

