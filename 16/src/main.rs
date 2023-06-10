mod packet;

use packet::PacketFactory;

use std::{fs::File, io::BufRead, io::BufReader};

fn parse(filepath: &str) -> Vec<bool> {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        // Convert each char into a 4 bit array of bool
        .chars()
        .map(|c| {
            let value = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(|i| (value & (1 << i)) != 0).collect()
        })
        .reduce(|acc, e| [acc, e].concat())
        .unwrap()
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let bitstream = parse(&filepath);
    let root_packet = PacketFactory::factory(&bitstream);

    println!(
        "Problem #1: {}",
        root_packet
            .all_versions()
            .into_iter()
            .map(|v| v as u64)
            .sum::<u64>()
    );
    println!("Problem #2: {:?}", root_packet.compute());
}
