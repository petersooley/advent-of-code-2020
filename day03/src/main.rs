use std::io::BufRead;
use std::{env, fs, io, fmt};


struct Slope {
    right: usize,
    down: usize,
    trees: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        assert!(right > 0 && down > 0);
        Self { right, down, trees: 0 }
    }

    fn scan(&mut self, l: &str, row: usize) {
        if row == 0 || row % self.down != 0 {
            return
        }

        // off-by-one works out just fine here since starting point is 1,1
        if l.chars().nth((row * self.right) % l.len()).unwrap() == '#' {
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
