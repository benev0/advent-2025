use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day4.txt")?;
    let reader = BufReader::new(file);

    let mut field: Vec<Vec<i8>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let line: Vec<i8> = line
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'.' => 0,
                b'@' => 1,
                _ => unreachable!(),
            })
            .collect();

        field.push(line);
    }

    let mut count = 0;

    loop {
        flood_field(&mut field);
        let nc = count_field(&field);

        print_filed(&field);

        if nc == 0 {
            break;
        }

        count += nc;

        reset_field(&mut field);
        println!();
        print_filed(&field);
    }

    println!("counted {} available papers", count);

    Ok(())
}

fn print_filed(field: &Vec<Vec<i8>>) {
    field.iter().for_each(|row| {
        row.iter().for_each(|elem| {
            print!("{}", *elem)
        });
        println!();
    });
}

fn measure_field(field: &Vec<Vec<i8>>) -> (i32, i32) {
    (
        field.len() as i32,
        field[0].len() as i32,
    )
}

fn reset_field(field: &mut Vec<Vec<i8>>) {
    let (height, width) = measure_field(&field);

    for y in 0..height {
        for x in 0..width {
            let x = x as usize;
            let y = y as usize;
            if field[y][x] > 0 && field[y][x] > 4 {
                field[y][x] = 1;
            } else {
                field[y][x] = 0;
            }
        }
    }
}

fn count_field(field: &Vec<Vec<i8>>) -> usize {
    field.iter().map(|row|
        row.iter().filter(|elem|
            **elem > 0 && **elem <= 4
        ).count()
    ).sum()
}

fn flood_field(field: &mut Vec<Vec<i8>>) {
    let (height, width) = measure_field(&field);

    for y in 0i32..height {
        for x in 0i32..width {
            if get(field, x, y) != 0 {
                inc_nz(field, x-1, y-1);
                inc_nz(field, x-1, y);
                inc_nz(field, x-1, y+1);
                inc_nz(field, x,   y-1);

                inc_nz(field, x,   y+1);
                inc_nz(field, x+1, y-1);
                inc_nz(field, x+1, y);
                inc_nz(field, x+1, y+1);
            }
        }
    }
}

#[inline(always)]
fn get(field: &Vec<Vec<i8>>, x: i32, y: i32) -> i8 {
    if x < 0 || y < 0 {
        return 0;
    }
    let x = x as usize;
    let y = y as usize;
    *field
        .get(y)
        .and_then(|row: &Vec<i8>|
            row.get(x)
        ).unwrap_or(&0)
}

#[inline(always)]
fn inc_nz(field: &mut Vec<Vec<i8>>, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    field
        .get_mut(y)
        .and_then(|row|
            row
                .get_mut(x)
                .map(|val| {
                    if *val != 0 {
                        *val += 1;
                    }
                }
            )
        );
}
