use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut joltage: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        let array = line.as_bytes();

        joltage += compute_max_joltage(array) as u32;
    }

    println!("counted {} max joltage", joltage);
    Ok(())
}

fn compute_max_joltage(array: &[u8]) -> u8 {
    let a = array[..array.len()-1]
        .iter()
        .enumerate()
        .fold((0usize,0u8), |acc, new|
            match *new.1 > acc.1 {
                true => (new.0, *new.1),
                false => acc,
            }
        );

    let b = array[a.0 + 1..]
        .iter()
        .max()
        .expect("array expects at least one element at this point");

    (a.1 - b'0') * 10 + b - b'0'
}