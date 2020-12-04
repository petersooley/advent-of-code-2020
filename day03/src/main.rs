use std::io::BufRead;
use std::{env, fmt, fs, io};

struct Slope {
    right: usize,
    down: usize,
    trees: usize,
    scans: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        assert!(right > 0 && down > 0);
        Self {
            right,
            down,
            scans: 0,
            trees: 0,
        }
    }

    fn scan(&mut self, l: &str, row: usize) {
        if row == 0 || row % self.down != 0 {
            return;
        }

        // keep track of which iteration we're at (of relevant rows)
        self.scans += 1;

        // off-by-one works out just fine here since starting point is 1,1
        if l.chars().nth((self.scans * self.right) % l.len()).unwrap() == '#' {
            self.trees += 1;
        }
    }
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}d{}: {}", self.right, self.down, self.trees)
    }
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("missing filename");

    let f = fs::File::open(&filename)?;
    let reader = io::BufReader::new(f);

    let mut slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    let lines = reader
        .lines()
        .map(|l| l.expect("failed to read line"))
        .enumerate();
    for (row, line) in lines {
        for slope in slopes.iter_mut() {
            slope.scan(&line, row)
        }
    }

    let product = slopes.iter().fold(1, |p, s| {
        println!("{}", s);
        p * s.trees
    });

    println!("product: {}", product);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::Enumerate;

    fn sample() -> Vec<&'static str> {
        vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
    }

    #[test]
    fn test_r1d1() {
        let mut s = Slope::new(1, 1);
        for (row, line) in sample().iter().enumerate() {
            s.scan(line, row)
        }

        assert_eq!(2, s.trees);
    }

    #[test]
    fn test_r3d1() {
        let mut s = Slope::new(3, 1);
        for (row, line) in sample().iter().enumerate() {
            s.scan(line, row)
        }

        assert_eq!(7, s.trees);
    }

    #[test]
    fn test_r5d1() {
        let mut s = Slope::new(5, 1);
        for (row, line) in sample().iter().enumerate() {
            s.scan(line, row)
        }

        assert_eq!(3, s.trees);
    }

    #[test]
    fn test_r7d1() {
        let mut s = Slope::new(7, 1);
        for (row, line) in sample().iter().enumerate() {
            s.scan(line, row)
        }

        assert_eq!(4, s.trees);
    }

    #[test]
    fn test_r1d2() {
        let mut s = Slope::new(1, 2);
        for (row, line) in sample().iter().enumerate() {
            s.scan(line, row)
        }

        assert_eq!(2, s.trees);
    }
}
