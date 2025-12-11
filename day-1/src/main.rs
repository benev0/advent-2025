use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day1.txt")?;
    let reader = BufReader::new(file);

    let mut dial: i64 = 50;
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let turn: i64 = line[1..].parse().unwrap();
        match line {
            line if line.starts_with("R") => {
                dial += turn;
            }
            line if line.starts_with("L") => {
                dial -= turn;
            }
            _ => panic!("not a lock seq")
        }
        if dial % 100 == 0 {
            count += 1;
        }
    }

    println!("counted {} zero clicks", count);
    Ok(())
}