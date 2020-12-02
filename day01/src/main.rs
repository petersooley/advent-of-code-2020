use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn sum_two(numbers: &Vec<u16>) -> Option<u64> {
    for j in numbers {
        for k in numbers {
            if j + k == 2020 {
                return Some((*j as u64) * (*k as u64))
            }
        }
    }
    None
}

fn sum_three(numbers: &Vec<u16>) -> Option<u64> {
    for j in numbers {
        for k in numbers {
            for l in numbers {
                if j + k + l == 2020 {
                    return Some((*j as u64) * (*k as u64) * (*l as u64))
                }
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("missing filename arg")
    }

    let f = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(f);

    let numbers: Vec<u16> = reader.lines()
      .map(|l|
        l.expect("failed to read line")
        .parse().expect("failed to parse line as u16")
      ).collect();

    if let Some(s) = sum_two(&numbers) {
        println!("{}", s);
    } else {
        println!("failed to find two lines with sum of 2020")
    }

    if let Some(s) = sum_three(&numbers) {
        println!("{}", s);
    } else {
        println!("failed to find three lines with sum of 2020")
    }

    Ok(())

}
