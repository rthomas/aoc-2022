use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);
    let mut max = 0;
    let mut acc = 0;

    for line in reader.lines() {
        match line?.as_str() {
            "" => {
                if acc > max {
                    max = acc;
                }
                acc = 0;
            }
            l => {
                acc += l.parse::<u32>()?;
            }
        }
    }
    println!("{max}");

    Ok(())
}
