use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> Vec<u16> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    // Create a new grid to be read from file (0,0) from empty grid
    reader
        .lines()
        .map(|l| u16::from_str_radix(&l.unwrap(), 2).unwrap())
        .collect()
}

const BITS: u8 = 12;
const MASKALL: u16 = 0b111111111111;

fn calculate_gamma(numbers: &Vec<u16>) -> u16 {
    // Each bit in the gamma rate can be determined by finding the most common
    // bit in the corresponding position of all numbers in the diagnostic report
    let half_length = numbers.len() as u64 / 2;
    let masks = (0..BITS).map(|i| 1u16 << i).collect::<Vec<_>>();

    // Make bit counts
    let mut counts = [0u64; BITS as usize];
    for &v in numbers {
        masks
            .iter()
            .enumerate()
            .filter(|(_, &m)| v & m != 0)
            .map(|(i, _)| counts[i] += 1)
            .for_each(drop);
    }

    let mut result = 0u16;
    masks
        .iter()
        .enumerate()
        .filter(|(i, _)| counts[*i] > half_length)
        .map(|(_, m)| result |= m)
        .for_each(drop);

    // result
    result & MASKALL
}

const fn calculate_epsilon(gamma: u16) -> u16 {
    // The epsilon rate is calculated in a similar way; rather than use the most
    // common bit, the least common bit from each position is used.
    !gamma & MASKALL
}

fn calculate_o2_co2(numbers: &Vec<u16>, is_o2: bool) -> u16 {
    let mut candidates = numbers.iter().map(|x| x).collect::<Vec<_>>();
    let masks = (0..BITS).map(|i| 1u16 << i).rev().collect::<Vec<_>>();

    for m in masks {
        // If there is only one candidate, then we cancel our search
        if candidates.len() == 1 {
            break;
        }

        // Count candidates for each bit
        let mut counts_zeros = Vec::new();
        let mut counts_ones = Vec::new();
        for c in candidates {
            if *c & m == 0 {
                counts_zeros.push(c);
            } else {
                counts_ones.push(c)
            }
        }

        // Update list of candidates with the most predominant
        candidates = if is_o2 {
            if counts_zeros.len() > counts_ones.len() {
                counts_zeros
            } else {
                counts_ones
            }
        } else {
            if counts_zeros.len() <= counts_ones.len() {
                counts_zeros
            } else {
                counts_ones
            }
        };
    }

    // Return the only candidate
    assert_eq!(candidates.len(), 1);
    **candidates.first().unwrap()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    // Parse file
    let numbers = parse(&filepath);
    let gamma = calculate_gamma(&numbers);
    let epsilon = calculate_epsilon(gamma);
    let o2 = calculate_o2_co2(&numbers, true);
    let co2 = calculate_o2_co2(&numbers, false);

    println!(
        "Part1: {:#b}({}) x {:#b}({}) = {}",
        gamma,
        gamma,
        epsilon,
        epsilon,
        u64::from(gamma) * u64::from(epsilon)
    );

    println!(
        "Part2: {:#b}({}) x {:#b}({}) = {}",
        o2,
        o2,
        co2,
        co2,
        u64::from(o2) * u64::from(co2)
    );
}
