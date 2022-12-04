use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);
    let mut max = 0;
    let mut acc = 0;

    let mut totals = Vec::new();

    for line in reader.lines() {
        match line?.as_str() {
            "" => {
                if acc > max {
                    max = acc;
                }
                totals.push(acc);
                acc = 0;
            }
            l => {
                acc += l.parse::<u32>()?;
            }
        }
    }
    println!("Part 1: {max}");

    assert!(totals.len() >= 3);
    totals.sort_by(|a, b| b.cmp(a));
    println!("Part 2: {}", totals[0] + totals[1] + totals[2]);

    Ok(())
}
