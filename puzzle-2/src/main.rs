use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./input")?;
    let reader = BufReader::new(input);
    let mut part1_score: u32 = 0;
    let mut part2_score: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut vals = line.split(" ");
        let them = vals.next().expect("invalid input").into();
        let col2 = vals.next().expect("invalid input");
        let me = col2.into();
        part1_score += get_score(them, me) as u32;

        // part 2
        part2_score += match col2 {
            "X" => {
                // lose
                let me = &match them {
                    RPS::Rock => RPS::Scissors,
                    RPS::Paper => RPS::Rock,
                    RPS::Scissors => RPS::Paper,
                };
                get_score(them, me)
            }
            "Y" => {
                // draw
                get_score(them, them)
            }
            "Z" => {
                // win
                let me = &match them {
                    RPS::Rock => RPS::Paper,
                    RPS::Paper => RPS::Scissors,
                    RPS::Scissors => RPS::Rock,
                };
                get_score(them, me)
            }
            _ => panic!("invalid input: {col2}"),
        } as u32;
    }

    println!("Part 1: {part1_score}");
    println!("Part 2: {part2_score}");

    Ok(())
}

fn get_score(them: &RPS, me: &RPS) -> u8 {
    if them > me {
        // loss
        me.score()
    } else if me > them {
        // win
        6 + me.score()
    } else {
        // draw
        3 + me.score()
    }
}

#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u8 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => Some(Ordering::Equal),
            (RPS::Rock, RPS::Paper) => Some(Ordering::Less),
            (RPS::Rock, RPS::Scissors) => Some(Ordering::Greater),
            (RPS::Paper, RPS::Rock) => Some(Ordering::Greater),
            (RPS::Paper, RPS::Paper) => Some(Ordering::Equal),
            (RPS::Paper, RPS::Scissors) => Some(Ordering::Less),
            (RPS::Scissors, RPS::Rock) => Some(Ordering::Less),
            (RPS::Scissors, RPS::Paper) => Some(Ordering::Greater),
            (RPS::Scissors, RPS::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl From<&str> for &RPS {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => &RPS::Rock,
            "B" | "Y" => &RPS::Paper,
            "C" | "Z" => &RPS::Scissors,
            _ => panic!("unexpected value: {s}"),
        }
    }
}
