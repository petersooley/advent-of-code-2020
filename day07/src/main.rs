use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn parse_rule(line: &str) -> (String, Vec<String>) {
    let mut children: Vec<String> = vec![];

    let mut parts = line.split_ascii_whitespace();
    let mut bag = String::from(parts.next().expect("missing adj of bag name"));
    bag.push_str(" ");
    bag.push_str(parts.next().expect("missing color of bag name"));

    parts.next().expect("missing 'bags' marker of bag");
    parts.next().expect("missing 'contains' marker");

    while let Some(count) = parts.next() {
        if count.starts_with("no") {
            break;
        }

        let mut child = String::from(parts.next().expect("missing adj of child name"));
        child.push_str(" ");
        child.push_str(parts.next().expect("missing color of child name"));
        children.push(child);

        parts.next().expect("missing 'bags' marker of child");
    }

    (bag, children)
}

fn find_parents(lookup: &HashMap<&String, Vec<&String>>, found: &mut HashSet<String>, child: &String) {
    // skip items we've already counted
    if found.contains(child) {
        return;
    }

    found.insert(child.clone());

    // recursive count
    if let Some(direct_parents) = lookup.get(child) {
        // println!("{} => {:?}", child, direct_parents);

        for parent in direct_parents {
            find_parents(lookup, found, parent);
        }
    }
}

fn count_parents(lookup: &HashMap<&String, Vec<&String>>, child: &str) -> usize {
    let mut found: HashSet<String> = HashSet::new();
    find_parents(lookup, &mut found, &String::from(child));
    found.len() - 1  // exclude initial child
}

fn flip_rules(rules: &HashMap<String, Vec<String>>) -> HashMap<&String, Vec<&String>> {
    let mut lookup: HashMap<&String, Vec<&String>> = HashMap::new();
    for (parent, children) in rules.iter() {
        for child in children.iter() {
            let entry = lookup.entry(child).or_insert_with(|| vec![]);
            entry.push(parent);
        }
    }
    lookup
}

fn main() {
    // rules: parent => children
    let rules: HashMap<String, Vec<String>> = INPUT.lines().map(|l| parse_rule(l)).collect();

    // rules flipped: child => parents (direct)
    let mut lookup = flip_rules(&rules);

    let our_bag = String::from("shiny gold");

    println!("{} bag colors can contain 'shiny gold' bags", count_parents(&lookup, &our_bag));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;


    #[test]
    fn test_simple() {
        let sample = HashMap::from_iter(vec![
            (String::from("blue"), vec![String::from("red"), String::from("green")]),
        ]);
        let lookup = flip_rules(&sample);

        assert_eq!(count_parents(&lookup, "red"), 1);
        assert_eq!(count_parents(&lookup, "green"), 1);
        assert_eq!(count_parents(&lookup, "blue"), 0);
    }

    #[test]
    fn test_deeper() {
        let sample = HashMap::from_iter(vec![
            (String::from("blue"), vec![String::from("red"), String::from("green")]),
            (String::from("red"), vec![String::from("orange")]),
        ]);
        let lookup = flip_rules(&sample);

        assert_eq!(count_parents(&lookup, "red"), 1);
        assert_eq!(count_parents(&lookup, "green"), 1);
        assert_eq!(count_parents(&lookup, "blue"), 0);
        assert_eq!(count_parents(&lookup, "orange"), 2);
    }

    #[test]
    fn test_cycles() {
        let sample = HashMap::from_iter(vec![
            (String::from("blue"), vec![String::from("red"), String::from("green")]),
            (String::from("red"), vec![String::from("orange")]),
            (String::from("green"), vec![String::from("orange")]),
        ]);
        let lookup = flip_rules(&sample);

        assert_eq!(count_parents(&lookup, "red"), 1);
        assert_eq!(count_parents(&lookup, "green"), 1);
        assert_eq!(count_parents(&lookup, "blue"), 0);
        assert_eq!(count_parents(&lookup, "orange"), 3);
    }
}
