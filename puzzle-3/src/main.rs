use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);

    let mut acc = 0;

    let mut p2_acc = 0;
    let mut p2_lines = Vec::new();

    for line in reader.lines() {
        let line = line?;

        p2_lines.push(line.chars().collect::<HashSet<char>>());

        let parts = line.split_at(line.len() / 2);
        let comp1 = parts.0.chars().collect::<HashSet<char>>();
        let comp2 = parts.1.chars().collect::<HashSet<char>>();

        let mut intersection = comp1.intersection(&comp2);
        let c = intersection
            .next()
            .expect("should be a single char overlap");

        acc += get_priority(c);
    }

    println!("Part 1: {acc}");

    for triple in p2_lines.as_slice().chunks(3) {
        let i = triple[0]
            .intersection(&triple[1])
            .cloned()
            .collect::<HashSet<char>>();
        let mut i = i.intersection(&triple[2]);

        let c = i.next().expect("no intersection found");
        p2_acc += get_priority(c);
    }

    println!("Part 2: {p2_acc}");

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
