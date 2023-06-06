use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split("|")
                .map(|v| v.trim().to_owned())
                .collect_tuple()
                .unwrap()
        })
        .map(|(i, o)| {
            (
                i.split(" ").map(|v| v.to_owned()).collect(),
                o.split(" ").map(|v| v.to_owned()).collect(),
            )
        })
        .collect()
}

// Consumes a signal (series of letters contaning digits) and outputs
// the wiring (number of wires + (each digit represents a segment))
// e.g. "badf" -> (4, 43)
fn signal_to_wiring(data: &str) -> (u8, u8) {
    (
        data.len() as u8,
        data.chars()
            .map(|c| 1 << (c as u8 - 'a' as u8))
            .reduce(|a, b| a | b)
            .unwrap_or(0u8),
    )
}

fn wiring_to_decode(data: &Vec<String>) -> [u8; 10] {
    // There are:
    // * one   digit  with 2 segments: 1
    // * one   digit  with 3 segments: 7
    // * one   digit  with 4 segments: 4
    // * one   digit  with 7 segments: 8
    // * three digits with 5 segments: 2, 3, 5
    // * three digits with 6 segments: 0, 6, 9
    // e.g. [2, 3, 4, 5, 5, 5, 6, 6, 6, 7]
    let mut result = [0u8; 10];
    let mut data_wires: Vec<_> = data.into_iter().map(|c| signal_to_wiring(c)).collect();
    data_wires.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

    // Digit 1 has only two segments, (it's always the first segment)
    result[1] = data_wires[0].1;

    // Digit 7 has only three segments, (it's always the second segment)
    result[7] = data_wires[1].1;

    // Digit 4 has only four segments, (it's always the third segment)
    result[4] = data_wires[2].1;

    // Digit 8 has all segments, (it's always the last segment)
    result[8] = data_wires.last().unwrap().1;

    // Digit 3 is the only 5 segment (digit that remains the same with digit 1
    for index in 3..6 {
        if data_wires[index].1 & result[1] == result[1] {
            result[3] = data_wires[index].1;
            break;
        }
    }

    // Digit 9 is the join between 4 and 3
    result[9] = result[3] | result[4];

    // Digit 5 is the same as 9 when joined with 9, excepting 3
    for index in 3..6 {
        if (data_wires[index].1 | result[9] == result[9]) && (data_wires[index].1 != result[3]) {
            result[5] = data_wires[index].1;
            break;
        }
    }

    // Digit 2 is the remaining 5 segment digit
    for index in 3..6 {
        if (data_wires[index].1 != result[5]) && (data_wires[index].1 != result[3]) {
            result[2] = data_wires[index].1;
            break;
        }
    }

    // Digit 6: if digit joined with 1 is 8, then this digit is 6
    for index in 6..9 {
        if data_wires[index].1 | result[1] == result[8] {
            result[6] = data_wires[index].1;
            break;
        }
    }

    // Digit 0: is the remaining 6 segment digit
    for index in 6..9 {
        if (data_wires[index].1 != result[9]) && (data_wires[index].1 != result[6]) {
            result[0] = data_wires[index].1;
            break;
        }
    }

    result
}

fn decode_digit(map: &[u8; 10], input: &str) -> Option<u8> {
    let (_, wiring) = signal_to_wiring(input);

    for (i, d) in map.iter().enumerate() {
        if wiring == *d {
            return Some(i as u8);
        }
    }

    None
}

fn decode_numbers(map: &[u8; 10], digits: &Vec<String>) -> u64 {
    digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, s)| decode_digit(map, s).unwrap() as u64 * 10u64.pow(i as u32))
        .sum()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");
    let data = parse(&filepath);

    // Problem #1
    println!(
        "Problem #1: {}",
        &data
            .iter()
            .map(|(_, d)| {
                d.into_iter()
                    .map(|c| c.len())
                    .filter(|&c| c == 2 || c == 3 || c == 4 || c == 7)
                    .count()
            })
            .sum::<usize>()
    );

    // Problem #2
    println!(
        "Problem #2: {}",
        data.into_iter()
            .map(|(din, dout)| decode_numbers(&wiring_to_decode(&din), &dout))
            .sum::<u64>()
    );
}
