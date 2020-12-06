use std::collections::HashSet;
use std::io::Read;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("missing filename");

    let f = fs::File::open(&filename)?;

    let mut reader = io::BufReader::new(f);
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .expect("failed to read contents to string");

    let sum_anyone: usize = buf
        .split("\n\n")
        .map(|group| {
            let set: HashSet<char> = group.chars().filter(|c| !c.is_ascii_whitespace()).collect();
            set.len()
        })
        .sum();

    let sum_everyone: usize = buf
        .split("\n\n")
        .map(|group| {
            let mut sets = group.lines().map(|l| l.chars().collect::<HashSet<char>>());
            let first: HashSet<char> = sets.next().expect("group has no members");
            let set: HashSet<char> = sets.fold(first, |a, b| a.intersection(&b).cloned().collect());
            set.len()
        })
        .sum();

    println!("total questions anyone answered {}", sum_anyone);
    println!("total questions everyone answered {}", sum_everyone);

    Ok(())
}
