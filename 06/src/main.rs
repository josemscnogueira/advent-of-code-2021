use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> [usize; 9] {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    let mut result = [0usize; 9];
    for &value in reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
        .last()
        .unwrap()
    {
        result[value as usize] += 1;
    }

    result
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let mut allfish = parse(&filepath);
    println!("Day 0 fish: {:?}", allfish);

    for d in 1..=256 {
        let prevday = std::mem::replace(&mut allfish, [0usize; 9]);

        for index in 0..8 {
            allfish[index] = prevday[index + 1];
        }
        allfish[8] = prevday[0];
        allfish[6] += prevday[0];

        println!("Day {} fish: {:?}", d, allfish);

        if d == 80 {
            println!("Part1: {}", allfish.iter().sum::<usize>());
        }
    }

    println!("Part2: {}", allfish.iter().sum::<usize>());
}
