use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);

    let mut acc = 0;

    for line in reader.lines() {
        let line = line?;
        let parts = line.split_at(line.len() / 2);

        let mut comp1 = HashSet::with_capacity(line.len() / 2);
        for c in parts.0.chars() {
            comp1.insert(c);
        }

        let mut comp2 = HashSet::with_capacity(line.len() / 2);
        for c in parts.1.chars() {
            comp2.insert(c);
        }

        let mut intersection = comp1.intersection(&comp2);
        let c = intersection
            .next()
            .expect("should be a single char overlap");

        acc += get_priority(c);
    }

    println!("Part 1: {acc}");

    Ok(())
}

fn get_priority(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        *c as u32 - 96
    } else if c.is_ascii_uppercase() {
        *c as u32 - 38
    } else {
        panic!("invalid character: {c}")
    }
}
