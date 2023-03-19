use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> Vec<usize> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .last()
        .unwrap()
        .to_vec()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let crabs = {
        let mut a = parse(&filepath);
        a.sort();
        a
    };
    let crab_min = *crabs.iter().min().unwrap();
    let crab_max = *crabs.iter().max().unwrap();

    println!(
        "Part 1:  Consumption: {:}",
        (crab_min..=crab_max)
            .map(|c| {
                crabs
                    .iter()
                    .map(|&v| ((v as isize) - (c as isize)).abs())
                    .sum::<isize>()
            })
            .min()
            .unwrap()
    );

    println!(
        "Part 2:  Consumption: {:}",
        (crab_min..=crab_max)
            .map(|c| {
                crabs
                    .iter()
                    .map(|&v| ((v as isize) - (c as isize)).abs())
                    .map(|v| v * (v + 1) / 2)
                    .sum::<isize>()
            })
            .min()
            .unwrap()
    );
}
