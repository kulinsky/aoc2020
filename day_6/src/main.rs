use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut set: HashSet<char> = HashSet::new();
    let mut answers: HashMap<char, usize> = HashMap::new();
    let mut person_counter: usize = 0;

    for group in read_to_string("src/input.txt").unwrap().split("\n\n") {
        for person in group.split("\n").filter(|x| !x.is_empty()) {
            person_counter += 1;
            for letter in person.chars() {
                set.insert(letter);
                *answers.entry(letter).or_default() += 1;
            }
        }
        part_1 += set.len();
        set.clear();
        part_2 += answers.values().filter(|&&v| v == person_counter).count();
        answers.clear();
        person_counter = 0;
    }
    println!("Elapsed: {}ms", now.elapsed().as_millis());
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
