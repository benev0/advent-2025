fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("./input/day2.txt")?;

    let ids: Vec<std::ops::RangeInclusive<u64>> = data.split(",")
        .map_while(|s|
            s.split_once('-')
                .and_then(|tup| Some( tup.0.parse().ok()? ..= tup.1.parse().ok()? ))
        ).collect();

    let sum: u64 = ids.into_iter()
        .map(|range| {
            range.fold(0u64, |acc, val| {
                match check_invalid(val) {
                    true => {
                        acc + val as u64
                    },
                    false => {
                        acc
                    },
                }
            })
        }
        )
        .sum();

    println!("the sum of invalid fragments is {}", sum);
    Ok(())
}

fn check_invalid(pid: u64) -> bool {
    let len = pid.ilog10() + 1;
    match len {
        // one false
        2 =>  pid % 11 == 0,
        4 =>  pid % 0101 == 0,
        6 =>  pid % 001001 == 0,
        8 =>  pid % 00010001 == 0,
        10 => pid % 0000100001 == 0,
        _ => false,
    }
}
