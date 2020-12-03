use regex::{Captures, Regex};
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn split_line(s: &str) -> Result<Captures, String> {
    let re = Regex::new(r"(\d+)-(\d+)\s+([a-z]):\s+([a-z]+)")
        .map_err(|_| String::from("failed to create regex"))?;

    let caps = re
        .captures(s)
        .ok_or_else(|| format!("regex failed to find captures any line: '{}'", s))?;

    if caps.len() == 0 {
        return Err(format!("not enough captures found in line: '{}'", s));
    }

    Ok(caps)
}

#[derive(Debug)]
struct PasswordEntry {
    left: usize,
    right: usize,
    target: char,
    password: String,
}

impl PasswordEntry {
    fn target_count(&self) -> usize {
        self.password.matches(self.target).count()
    }

    fn left_char(&self) -> Option<char> {
        self.password.chars().nth(self.left - 1)
    }

    fn right_char(&self) -> Option<char> {
        self.password.chars().nth(self.right - 1)
    }
}

impl FromStr for PasswordEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = split_line(s)?;

        let left = caps
            .get(1)
            .ok_or_else(|| format!("no left value found in line: '{}'", s))?
            .as_str()
            .parse::<usize>()
            .map_err(|e| {
                format!(
                    "failed to parse usize left from line: '{}'. error: {}",
                    s, e
                )
            })?;

        let right = caps
            .get(2)
            .ok_or_else(|| format!("no right value found in line: '{}'", s))?
            .as_str()
            .parse::<usize>()
            .map_err(|e| {
                format!(
                    "failed to parse usize right from line: '{}'. error: {}",
                    s, e
                )
            })?;

        let target = caps
            .get(3)
            .ok_or_else(|| format!("no target char found in line: '{}'", s))?
            .as_str()
            .chars()
            .next()
            .ok_or_else(|| format!("no chars in target string found for line: '{}'", s))?;

        let password = caps
            .get(4)
            .ok_or_else(|| format!("no password found in line: '{}", s))?
            .as_str()
            .to_string();

        Ok(PasswordEntry {
            left,
            right,
            target,
            password,
        })
    }
}

fn by_count(entry: &PasswordEntry) -> Option<()> {
    let count = entry.target_count();
    if count > entry.right || count < entry.left {
        return None;
    }
    Some(())
}

fn by_index(entry: &PasswordEntry) -> Option<()> {
    let l = entry.left_char().expect("invalid left index");
    let r = entry.right_char().expect("invalid right index");
    if l != r && (l == entry.target || r == entry.target) {
        return Some(());
    }
    return None;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("missing filename arg")
    }

    let f = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(f);

    let results = reader
        .lines()
        .fold((0 as u64, 0 as u64), |acc, l| -> (u64, u64) {
            let entry = l
                .expect("failed to read line")
                .parse::<PasswordEntry>()
                .unwrap();

            (
                acc.0 + by_count(&entry).map_or(0, |_| 1),
                acc.1 + by_index(&entry).map_or(0, |_| 1),
            )
        });

    println!("{}", results.0);
    println!("{}", results.1);

    Ok(())
}
