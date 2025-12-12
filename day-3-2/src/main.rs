use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut joltage: u64 = 0;

    for line in reader.lines() {
        let line = line?;

        let array: Vec<u8> = line
            .as_bytes()
            .iter()
            .map(|val| val - b'0')
            .collect();

        joltage += compute_max_joltage(&array);
    }

    println!("counted {} max joltage", joltage);
    Ok(())
}

fn compute_max_joltage(array: &Vec<u8>) -> u64 {

    let mut window_start = 0;
    let mut joltage: u64 = 0;

    for i in 0..12 {
        joltage *= 10;

        let a = array[window_start..array.len()-11+i]
            .iter()
            .enumerate()
            .fold((0usize,0u8), |acc, new|
                match *new.1 > acc.1 {
                    true => (new.0, *new.1),
                    false => acc,
                }
            );

        joltage += a.1 as u64;

        window_start += a.0 + 1;
    }

    // let b = array[a.0 + 1..]
    //     .iter()
    //     .max()
    //     .expect("array expects at least one element at this point");

    dbg!(joltage)
}