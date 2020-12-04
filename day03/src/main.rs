use std::io::BufRead;
use std::{env, fmt, fs, io};

struct Slope {
    right: usize,
    down: usize,
    trees: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        assert!(right > 0 && down > 0);
        Self {
            right,
            down,
            trees: 0,
        }
    }

    /// I had to break this down a bit. Ultimately, we are looking for the index of the target
    /// character in our current line (with some modulo magic because the lines are infinitely
    /// repeated rightward). We _could_ add together all the characters we've scanned over from
    /// each iteration or we could do math (basic algebra).
    ///
    /// First, we skip any rows that we know are irrelevant (including the first row, since that's
    /// where we start).
    ///
    /// Then we need to figure out which relevant row we're on: `row / self.down`. This is a factor
    /// we can multiply by `self.right` to figure out how far we need to look in our current line to
    /// get the target character. If down is `2`, then the first relevant row is `2` and
    /// `2 / 2 = 1`, the second relevant row is `4` and `4 / 2 = 2`, and so on.
    fn scan(&mut self, line: &str, row: usize) {
        if row == 0 || row % self.down != 0 {
            return;
        }

        if line
            .chars()
            .nth(((row / self.down) * self.right) % line.len())
            .unwrap()
            == '#'
        {
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
