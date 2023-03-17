use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum SubmarineControl {
    Up(u8),
    Down(u8),
    Forward(u8),
}

fn parse(filepath: &str) -> Vec<SubmarineControl> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    // Create a new grid to be read from file (0,0) from empty grid
    let mut result = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        let mut s = l.split(" ");

        result.push(match s.next().unwrap() {
            "forward" => SubmarineControl::Forward(s.next().unwrap().parse().unwrap()),
            "up" => SubmarineControl::Up(s.next().unwrap().parse().unwrap()),
            "down" => SubmarineControl::Down(s.next().unwrap().parse().unwrap()),
            _ => panic!("unexpected string"),
        });
    }

    result
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let mut position = (0i64, 0i64, 0i64);

    // Parse file
    let commands = parse(&filepath);

    // Calculate final position
    for c in &commands {
        match *c {
            SubmarineControl::Up(v) => position.1 -= i64::from(v),
            SubmarineControl::Down(v) => position.1 += i64::from(v),
            SubmarineControl::Forward(v) => position.0 += i64::from(v),
        }
    }

    println!(
        "Part1: {} x {} = {}",
        position.0,
        position.1,
        position.0 * position.1
    );

    // New command interpretation
    position = (0i64, 0i64, 0i64);
    for c in &commands {
        match *c {
            SubmarineControl::Up(v) => position.2 -= i64::from(v),
            SubmarineControl::Down(v) => position.2 += i64::from(v),
            SubmarineControl::Forward(v) => {
                let value = i64::from(v);
                position.0 += value;
                position.1 += value * position.2;
            }
        }
    }

    println!(
        "Part2: {} x {} = {}",
        position.0,
        position.1,
        position.0 * position.1
    );
}
