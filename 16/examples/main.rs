use std::{fs::File, io::BufRead, io::BufReader, ops::BitOr};

fn parse(filepath: &str) -> Vec<bool> {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| {
            let value = c.to_digit(16).unwrap() as u8;
            (0..8u8).rev().map(|i| (value & (1 << i)) != 0).collect()
        })
        .reduce(|acc, e| [acc, e].concat())
        .unwrap()
}

#[derive(Debug)]
struct PacketHeader<'a> {
    version: u8,
    id: u8,
    content: &'a [bool],
}

impl<'a> PacketHeader<'a> {
    fn init(data: &'a [bool]) -> Self {
        let version = (0..3)
            .map(|i| u8::from(data[i + 0]) << (2 - i))
            .reduce(|acc, e| acc.bitor(e))
            .unwrap();
        let id = (0..3)
            .map(|i| u8::from(data[i + 3]) << (2 - i))
            .reduce(|acc, e| acc.bitor(e))
            .unwrap();

        Self {
            version,
            id,
            content: &data[6..],
        }
    }
}

trait Packet {
    fn children<'a, 'b>(&'a self) -> Vec<Box<dyn Packet + 'b>>
    where
        'a: 'b;
    fn len(&self) -> usize;
}

struct PacketLiteral<'a> {
    header: PacketHeader<'a>,
    value: u64,
    length: usize,
}

impl<'a> PacketLiteral<'a> {
    fn init(header: PacketHeader<'a>) -> Self {
        debug_assert!(header.id == 4);

        let mut count = 0;
        let mut data = Vec::new();
        loop {
            data.extend_from_slice(&header.content[(count * 5 + 1)..((count + 1) * 5)]);
            count += 1;

            if !header.content[count * 5] {
                break;
            };
        }

        let length = data.len();
        let value = data
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, d)| u64::from(d) << (length - 1 - i))
            .reduce(|acc, e| acc.bitor(e))
            .unwrap();

        Self {
            header,
            value,
            length: count * 5 + 6,
        }
    }
}

impl Packet for PacketLiteral<'_> {
    fn children<'a, 'b>(&'a self) -> Vec<Box<dyn Packet + 'b>>
    where
        'a: 'b,
    {
        Vec::new()
    }

    fn len(&self) -> usize {
        self.length
    }
}

struct PacketOperator<'a> {
    header: PacketHeader<'a>,
    length_type: bool,
    length_bitsize: u8,
    length_value: u16,
}

impl<'a> PacketOperator<'a> {
    fn init(header: PacketHeader<'a>) -> Self {
        debug_assert!(header.id != 4);

        let length_bitsize = if header.content[0] { 15 } else { 11 };

        Self {
            length_bitsize,
            length_type: header.content[0],
            length_value: header.content[1..=length_bitsize as usize]
                .iter()
                .rev()
                .enumerate()
                .map(|(i, d)| u16::from(*d) << (length_bitsize as usize - 1 - i))
                .reduce(|acc, e| acc.bitor(e))
                .unwrap(),
            header,
        }
    }
}

impl Packet for PacketOperator<'_> {
    fn children<'a, 'b>(&'a self) -> Vec<Box<dyn Packet + 'b>>
    where
        'a: 'b,
    {
        let mut result: Vec<Box<dyn Packet + 'b>> = Vec::new();
        let mut cursor = 0usize;

        loop {
            // Stop if number of bits or number of packets was achieved, depending
            // on length_type
            if (!self.length_type && (cursor >= self.length_value as usize))
                || (self.length_type && (result.len() >= self.length_value as usize))
            {
                break;
            }

            // Create header for child packet, starting at 'cursor'
            let header = PacketHeader::init(
                &self.header.content[(self.length_bitsize as usize + 1 + cursor)..],
            );

            // Mini factory
            result.push(if header.id == 4 {
                Box::new(PacketLiteral::<'b>::init(header))
            } else {
                Box::new(PacketOperator::<'b>::init(header))
            });

            // Update cursor with the last added packet size
            cursor += result.last().unwrap().len();
        }

        result
    }

    fn len(&self) -> usize {
        6 + 1
            + self.length_bitsize as usize
            + if self.length_type {
                self.header.content.len() - 1 - self.length_bitsize as usize
            } else {
                self.length_value as usize
            }
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let bitstream = parse(&filepath);

    println!("Problem #1: {:?}[{}]", bitstream, bitstream.len());
    println!("Problem #1: {:?}", PacketHeader::init(&bitstream));
}
