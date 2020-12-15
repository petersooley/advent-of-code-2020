use std::cmp::min;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

/// Structure holding all the bag rules
///
///```
/// outer => [(inner, qty), ..]
/// ```
#[derive(Default)]
struct Rules(HashMap<String, HashMap<String, usize>>);

/// Structure holding a lookup of which bags can contain which
///
/// ```
/// inner => [outer, ..]
/// ```
#[derive(Default)]
struct OuterLookup(HashMap<String, HashSet<String>>);

impl Rules {
    pub fn from_lines(lines: Vec<&str>) -> Rules {
        let mut rules = Rules::default();

        for line in lines {
            rules.parse_rule(line);
        }
        rules
    }

    pub fn parse_rule(&mut self, line: &str) {
        let mut parts = line.split_ascii_whitespace();
        let mut outer = String::from(parts.next().expect("missing adj of bag name"));
        outer.push_str(" ");
        outer.push_str(parts.next().expect("missing color of bag name"));

        parts.next().expect("missing 'bags' marker of bag");
        parts.next().expect("missing 'contains' marker");

        let rules = self.0.entry(outer).or_default();

        while let Some(count) = parts.next() {
            if count.starts_with("no") {
                // should only happen on first iteration if it's gonna happen
                break;
            }

            let qty = count.parse::<usize>().unwrap_or(0);

            let mut inner = String::from(parts.next().expect("missing adj of child name"));
            inner.push_str(" ");
            inner.push_str(parts.next().expect("missing color of child name"));

            rules.insert(inner, qty);

            parts.next().expect("missing 'bags' marker of child");
        }
    }

    fn to_parent_lookup(&self) -> OuterLookup {
        let mut lookup = OuterLookup::default();
        for (outer, inners) in self.0.iter() {
            for inner in inners.keys() {
                let entry = lookup.0.entry(inner.clone()).or_default();
                entry.insert(outer.clone());
            }
        }

        lookup
    }

    fn sum_qtys(&self, start: &str) -> usize {
        let mut sum = 1;
        if let Some(rules) = self.0.get(start) {
            for (inner, qty) in rules.iter() {
                sum += qty * self.sum_qtys(inner);
            }
        }
        sum
    }

    pub fn count_required(&self, start: &str) -> usize {
        self.sum_qtys(start) - 1
    }
}

impl OuterLookup {
    fn find_outers(&self, inner: &String, found: &mut HashSet<String>) {
        if found.contains(inner) {
            return;
        }

        found.insert(inner.clone());

        if let Some(direct_outers) = self.0.get(inner) {
            for outer in direct_outers {
                self.find_outers(outer, found);
            }
        }
    }

    pub fn count(&self, inner: &str) -> usize {
        let mut found = HashSet::default();
        self.find_outers(&String::from(inner), &mut found);
        found.len() - 1 // exclude initial child
    }
}

fn main() {
    let mut rules = Rules::default();
    INPUT.lines().for_each(|l| rules.parse_rule(l));

    let lookup = rules.to_parent_lookup();

    let our_bag = String::from("shiny gold");

    println!(
        "{} bags can contain 'shiny gold' bags",
        lookup.count(&our_bag)
    );

    println!(
        "'shiny gold' bags must contain a total of {} bags",
        rules.count_required("shiny gold")
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    fn sample1<'a>() -> Vec<&'a str> {
        vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
    }

    fn sample2<'a>() -> Vec<&'a str> {
        vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
    }

    #[test]
    fn test_cost() {
        let rules = Rules::from_lines(sample1());
        assert_eq!(32, rules.count_required("shiny gold"));
    }
    #[test]
    fn test_cost_deep() {
        let rules = Rules::from_lines(sample2());
        assert_eq!(126, rules.count_required("shiny gold"));
    }

    #[test]
    fn test_simple() {
        let rules = Rules::from_lines(vec![
            "sky blue bags contain 3 dark red bags, 1 mint green bag",
        ]);
        let lookup = rules.to_parent_lookup();

        assert_eq!(1, lookup.count("dark red"));
        assert_eq!(1, lookup.count("mint green"));
        assert_eq!(0, lookup.count("sky blue"));
    }

    #[test]
    fn test_deeper() {
        let rules = Rules::from_lines(vec![
            "sky blue bags contain 3 dark red bags, 1 mint green bag",
            "dark red bags contain 2 ugly brown bags",
            "ugly brown bags contain 4 dirty yellow bags",
        ]);
        let lookup = rules.to_parent_lookup();

        assert_eq!(3, lookup.count("dirty yellow"));
        assert_eq!(1, lookup.count("dark red"));
        assert_eq!(1, lookup.count("mint green"));
        assert_eq!(2, lookup.count("ugly brown"));
    }

    #[test]
    fn test_cycles() {
        let rules = Rules::from_lines(vec![
            "sky blue bags contain 3 dark red bags, 1 mint green bag",
            "dark red bags contain 2 ugly brown bags",
            "ugly brown bags contain 4 dirty yellow bags",
            "faded gray bags contains 4 dirty yellow bags",
        ]);
        let lookup = rules.to_parent_lookup();

        assert_eq!(4, lookup.count("dirty yellow"));
        assert_eq!(0, lookup.count("faded gray"));
    }
}
