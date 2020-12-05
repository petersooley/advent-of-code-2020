use std::io::BufRead;
use std::{cmp, env, fs, io};

fn seat_id(row: u8, col: u8) -> u16 {
    (row as u16 * 8) + col as u16
}

fn find_row(str: &str) -> u8 {
    let mut bin = str.replace("F", "0");
    bin = bin.replace("B", "1");
    u8::from_str_radix(bin.as_str(), 2).expect("failed to parse row as u8")
}

fn find_col(str: &str) -> u8 {
    let mut bin = str.replace("L", "0");
    bin = bin.replace("R", "1");
    u8::from_str_radix(bin.as_str(), 2).expect("failed to parse col as u8")
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("missing filename");

    let f = fs::File::open(&filename)?;
    let reader = io::BufReader::new(f);

    let seat_ids: Vec<u16> = reader
        .lines()
        .map(|l| {
            let line = l.expect("failed to read line");
            let (row_str, col_str) = line.split_at(7);
            seat_id(find_row(row_str), find_col(col_str))
        })
        .collect();

    let max_seat_id = seat_ids.iter()
        .fold(0 as u16, |max, seat_id| cmp::max(max, *seat_id));

    println!("Max seat id: {}", max_seat_id);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(357, seat_id(44, 5));
    }

    #[test]
    fn test_find_row() {
        assert_eq!(44, find_row("FBFBBFF"));
        assert_eq!(70, find_row("BFFFBBF"));
        assert_eq!(14, find_row("FFFBBBF"));
        assert_eq!(102, find_row("BBFFBBF"));
        assert_eq!(0, find_row("FFFFFFF"));
        assert_eq!(127, find_row("BBBBBBB"));
    }

    #[test]
    fn test_find_col() {
        assert_eq!(5, find_col("RLR"));
        assert_eq!(7, find_col("RRR"));
        assert_eq!(4, find_col("RLL"));
    }
}
