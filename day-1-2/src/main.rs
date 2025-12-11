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
        let mod_turn = turn % 100;

        count += turn / 100;
        match line {
            line if line.starts_with("R") => {
                dial += mod_turn;
                if dial > 99 {
                    count += 1;
                }
                dial %= 100;
            }
            line if line.starts_with("L") => {
                if dial == 0 {
                    count -= 1;
                }

                dial -= mod_turn;

                if dial <= 0 {
                    count += 1;
                }

                if dial < 0 {
                    dial += 100;
                }

            }
            _ => panic!("not a lock seq")
        }


    }

    println!("counted {} zero clicks", count);
    Ok(())
}