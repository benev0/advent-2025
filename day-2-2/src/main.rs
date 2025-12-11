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
        // prime
        2 =>  pid % 11 == 0,
        // prime
        3 =>  pid % 111 == 0,
        // 2
        4 =>  pid % 01_01 == 0,
        // prime
        5 =>  pid % 11111 == 0,
        // 2 3
        6 =>  pid % 001_001 == 0 || pid % 01_01_01 == 0,
        // prime
        7 =>  pid % 1111111 == 0,
        // 2
        8 =>  pid % 0001_0001 == 0,
        // 3
        9 =>  pid % 001_001_001 == 0,
        // 5 2
        10 => pid % 00001_00001 == 0 || pid % 01_01_01_01_01 == 0,
        _ => false,
    }
}
