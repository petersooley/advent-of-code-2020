use std::collections::BTreeMap;
use std::io::BufRead;
use std::{env, fmt, fs, io, str};

struct Passport {
    entries: BTreeMap<String, String>, // ordered for Display
}

impl Passport {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn add_entries_from_line(&mut self, line: &str) {
        line.split(' ').filter(|e| e.len() > 0).for_each(|e| self.add_entry(e));
    }

    fn add_entry(&mut self, entry: &str) {
        let mut parts = entry.split(':');
        let key = parts.next().expect(format!("invalid entry key format: '{}'", entry).as_str());
        let val = parts.next().expect(format!("invalid entry val format: '{}'", entry).as_str());

        if !Self::valid_required_key(key) {
            // just skipping invalid keys (including "cid")
            return;
        }

        self.entries.insert(String::from(key), String::from(val));
    }

    fn valid_required_key(key: &str) -> bool {
        matches!(key, "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid")
    }

    pub fn has_sufficient_entries(&self) -> bool {
        self.entries.len() == 7
    }

    pub fn has_valid_entries(&self) -> bool {
        self.has_sufficient_entries() && self.all_entries_valid()
    }

    fn all_entries_valid(&self) -> bool {
        self.entries.iter().all(|(k, v)| match k.as_str() {
            "byr" => v
                .parse::<usize>()
                .map(|y| y >= 1920 && y <= 2002)
                .unwrap_or(false),
            "iyr" => v
                .parse::<usize>()
                .map(|y| y >= 2010 && y <= 2020)
                .unwrap_or(false),
            "eyr" => v
                .parse::<usize>()
                .map(|y| y >= 2020 && y <= 2030)
                .unwrap_or(false),
            "hgt" => {
                if let Some(hstr) = v.strip_suffix("cm") {
                    return hstr
                        .parse::<usize>()
                        .map(|h| h >= 150 && h <= 193)
                        .unwrap_or(false);
                } else if let Some(hstr) = v.strip_suffix("in") {
                    return hstr
                        .parse::<usize>()
                        .map(|h| h >= 59 && h <= 76)
                        .unwrap_or(false);
                }
                false
            }
            "hcl" => v
                .strip_prefix("#")
                .and_then(|c| {
                    if c.chars().all(|ch| ch.is_ascii_digit() || ch.is_ascii_lowercase()) {
                        return u32::from_str_radix(c, 16).ok()
                    }
                    None
                })
                .map(|b| b <= 16777215)
                .unwrap_or(false),
            "ecl" => matches!(
                    v.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                ),
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
            _ => false,
        })
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.has_valid_entries() {
            "valid:     "
        } else if self.has_sufficient_entries() {
            "sufficient:"
        } else {
            "invalid:   "
        };
        write!(f, "{} ({}) {:?}", status, self.entries.len(), self.entries)
    }
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("missing filename");

    let f = fs::File::open(&filename)?;
    let reader = io::BufReader::new(f);

    let mut cur_passport = Passport::new();
    let mut sufficient_entries = 0;
    let mut valid_entries = 0;
    let mut total_passports = 0;

    let lines = reader.lines().map(|l| l.expect("failed to read line"));
    for line in lines {
        if line.is_empty() {
            total_passports += 1;
            if cur_passport.has_sufficient_entries() {
                sufficient_entries += 1;
            }
            if cur_passport.has_valid_entries() {
                valid_entries += 1;
            }
            println!("{}", cur_passport);

            cur_passport = Passport::new();
        }

        cur_passport.add_entries_from_line(&line);
    }

    // don't forget the last one!
    total_passports += 1;
    if cur_passport.has_sufficient_entries() {
        sufficient_entries += 1;
    }
    if cur_passport.has_valid_entries() {
        valid_entries += 1;
    }
    println!("{}", cur_passport);

    println!(
        "{} of {} passports have sufficient entries.",
        sufficient_entries, total_passports
    );
    println!(
        "{} of {} passports have valid entries.",
        valid_entries, total_passports
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn validate_entry(entry: &str) -> bool {
        let mut p = Passport::new();
        p.add_entry(entry);
        p.entries.len() == 1 && p.all_entries_valid()
    }

    fn sample_1() -> Vec<&'static str> {
        vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
        ]
    }

    fn sample_2() -> Vec<&'static str> {
        vec![
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
        ]
    }

    fn sample_3() -> Vec<&'static str> {
        vec![
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
        ]
    }

    fn sample_4() -> Vec<&'static str> {
        vec![
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
    }

    fn from_sample(sample: Vec<&str>) -> Passport {
        let mut cur_passport = Passport::new();
        for line in sample {
            cur_passport.add_entries_from_line(line)
        }
        println!("{}", cur_passport);
        cur_passport
    }

    #[test]
    fn test_sample_1() {
        let p = from_sample(sample_1());
        assert!(p.has_sufficient_entries())
    }

    #[test]
    fn test_sample_2() {
        let p = from_sample(sample_2());
        assert!(!p.has_sufficient_entries())
    }

    #[test]
    fn test_sample_3() {
        let p = from_sample(sample_3());
        assert!(p.has_sufficient_entries())
    }

    #[test]
    fn test_sample_4() {
        let p = from_sample(sample_4());
        assert!(!p.has_sufficient_entries())
    }

    #[test]
    fn test_add_entry() {
        let mut p = Passport::new();
        p.add_entry("byr:1937");
        p.add_entry("iyr:2017");
        p.add_entry("oops:uhoh");
        assert_eq!(p.entries.len(), 2);
        assert!(!p.has_sufficient_entries());
    }

    #[test]
    fn test_valid() {
        let mut p = Passport::new();
        p.add_entry("byr:1937");
        p.add_entry("iyr:2017");
        p.add_entry("ecl:gry");
        p.add_entry("pid:860033327");
        p.add_entry("eyr:2020");
        p.add_entry("hcl:#fffffd");
        p.add_entry("hgt:183cm");
        p.add_entry("cid:147");
        assert!(p.has_sufficient_entries());
    }

    #[test]
    fn test_invalid() {
        let mut p = Passport::new();
        p.add_entry("byr:1937");
        p.add_entry("iyr:2017");
        p.add_entry("ecl:gry");
        p.add_entry("pid:860033327");
        p.add_entry("eyr:2020");
        p.add_entry("hcl:#fffffd");
        // p.add_entry("hgt:183cm");
        p.add_entry("cid:147");
        assert!(!p.has_sufficient_entries());
    }

    #[test]
    fn test_valid_no_cid() {
        let mut p = Passport::new();
        p.add_entry("byr:1937");
        p.add_entry("iyr:2017");
        p.add_entry("ecl:gry");
        p.add_entry("pid:860033327");
        p.add_entry("eyr:2020");
        p.add_entry("hcl:#fffffd");
        p.add_entry("hgt:183cm");
        // p.add_entry("cid:147");
        assert!(p.has_sufficient_entries());
    }

    #[test]
    fn test_invalid_no_cid() {
        let mut p = Passport::new();
        p.add_entry("byr:1937");
        p.add_entry("iyr:2017");
        p.add_entry("ecl:gry");
        p.add_entry("pid:860033327");
        p.add_entry("eyr:2020");
        p.add_entry("hcl:#fffffd");
        // p.add_entry("hgt:183cm");
        // p.add_entry("cid:147");
        assert!(!p.has_sufficient_entries());
    }

    #[test]
    fn test_validation_byr() {
        assert!(validate_entry("byr:2002"));
        assert!(!validate_entry("byr:2002x"));
        assert!(!validate_entry("byr:x2002"));
        assert!(!validate_entry("byr:2003"));
        assert!(!validate_entry("byr:19999"));
        assert!(validate_entry("byr:1920"));
        assert!(!validate_entry("byr:1919"));
    }

    #[test]
    fn test_validation_iyr() {
        assert!(validate_entry("iyr:2010"));
        assert!(validate_entry("iyr:2020"));
        assert!(!validate_entry("iyr:2021"));
        assert!(!validate_entry("iyr:202*"));
        assert!(!validate_entry("iyr:2009"));
        assert!(!validate_entry("iyr:20009"));
    }

    #[test]
    fn test_validation_eyr() {
        assert!(validate_entry("eyr:2020"));
        assert!(validate_entry("eyr:2030"));
        assert!(!validate_entry("eyr:20300"));
        assert!(!validate_entry("eyr:"));
        // assert!(!validate_entry("eyr")); // not an issue with this dataset
        assert!(!validate_entry("eyr:2031"));
        assert!(!validate_entry("eyr:2019"));
    }

    #[test]
    fn test_validation_hgt() {
        assert!(validate_entry("hgt:60in"));
        assert!(validate_entry("hgt:190cm"));
        assert!(!validate_entry("hgt:190in"));
        assert!(!validate_entry("hgt:190"));
    }

    #[test]
    fn test_validation_hcl() {
        assert!(validate_entry("hcl:#123abc"));
        assert!(!validate_entry("hcl:#123abz"));
        assert!(!validate_entry("hcl:123abc"));
        assert!(validate_entry("hcl:#0525bb"));
        assert!(!validate_entry("hcl:#0525bB"));
        assert!(validate_entry("hcl:#ffffff"));
        assert!(!validate_entry("hcl:#fffffg"));
    }

    #[test]
    fn test_validation_ecl() {
        assert!(validate_entry("ecl:amb"));
        assert!(validate_entry("ecl:blu"));
        assert!(validate_entry("ecl:brn"));
        assert!(validate_entry("ecl:gry"));
        assert!(validate_entry("ecl:grn"));
        assert!(validate_entry("ecl:hzl"));
        assert!(validate_entry("ecl:oth"));
        assert!(!validate_entry("ecl:other"));
        assert!(!validate_entry("ecl: blu"));
    }

    #[test]
    fn test_validation_pid() {
        assert!(validate_entry("pid:000000001"));
        assert!(!validate_entry("pid:0123456789"));
    }
}
