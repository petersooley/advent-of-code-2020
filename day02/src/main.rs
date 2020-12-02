use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use regex::{Regex, Captures};


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
struct PasswordChecker {
    min: u8,
    max: u8,
    target: char,
    password: String,
}

impl FromStr for PasswordChecker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // lazy_static! {
        //     static ref re: Regex::new(r"(\d+)-(\d+)\s+([a-z]):\s+([a-z]+)")
        //           .map_err(|_| String::from("failed to create regex"))?;
        // }
        let caps = split_line(s)?;

        let min = caps
            .get(1)
            .ok_or_else(|| format!("no min value found in line: '{}'", s))?
            .as_str()
            .parse::<u8>()
            .map_err(|e| format!("failed to parse u8 min from line: '{}'. error: {}", s, e))?;

        let max = caps
            .get(2)
            .ok_or_else(|| format!("no max value found in line: '{}'", s))?
            .as_str()
            .parse::<u8>()
            .map_err(|e| format!("failed to parse u8 max from line: '{}'. error: {}", s, e))?;

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

        Ok(PasswordChecker {
            min,
            max,
            target,
            password,
        })
    }
}

impl PasswordChecker {
    fn is_valid(&self) -> Option<()> {
        let count = self.target_count();
        if count > self.max || count < self.min {
            return None;
        }
        Some(())
    }

    fn target_count(&self) -> u8 {
        self.password.matches(self.target).count() as u8
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("missing filename arg")
    }

    let f = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(f);

    let valid_count = reader
        .lines()
        .filter_map(|l| -> Option<()> {
            l.expect("failed to read line")
                .parse::<PasswordChecker>()
                .unwrap()
                .is_valid()
        })
        .count();

    println!("{}", valid_count);

    Ok(())
}
