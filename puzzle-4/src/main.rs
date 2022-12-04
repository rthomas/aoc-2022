use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);

    let mut p1_acc = 0;
    let mut p2_acc = 0;

    for line in reader.lines() {
        let line = line?;
        let mut vals = line.split(",");
        let r1 = parse_to_range(vals.next().expect("bad line format"))?;
        let r2 = parse_to_range(vals.next().expect("bad line format"))?;

        if (r1.contains(r2.start()) && r1.contains(r2.end()))
            || (r2.contains(r1.start()) && r2.contains(r1.end()))
        {
            p1_acc += 1;
        }

        if r1.contains(r2.start())
            || r1.contains(r2.end())
            || r2.contains(r1.start())
            || r2.contains(r1.end())
        {
            p2_acc += 1;
        }
    }

    println!("Part 1: {p1_acc}");
    println!("Part 2: {p2_acc}");

    Ok(())
}

/// Parse a string in the form `x-y` into an inclusive range x to y.
fn parse_to_range(r: &str) -> Result<RangeInclusive<u32>, Box<dyn Error>> {
    let (x, y) = {
        let mut split = r.split("-");
        (
            split.next().expect("invalid format"),
            split.next().expect("invalid format"),
        )
    };

    Ok(RangeInclusive::new(x.parse()?, y.parse()?))
}
