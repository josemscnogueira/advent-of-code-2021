use nalgebra::{SMatrix, SVector};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn poly_parse(filepath: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    let mut template = Vec::new();
    let mut rules = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        match index {
            0 => template.extend(line.unwrap().chars()),
            _ => {
                let line = line.unwrap();
                if !line.is_empty() {
                    let conversion = line.split(" -> ").map(|c| c.to_string()).collect::<Vec<_>>();
                    match conversion.len() {
                        0 => (),
                        2 => {
                            _ = rules.insert(
                                conversion[0].chars().collect::<Vec<_>>().try_into().unwrap(),
                                conversion[1].chars().next().unwrap(),
                            )
                        }
                        c => panic!("Line {}:{} with unknown format ({} splits)", filepath, index + 1, c),
                    }
                }
            }
        }
    }

    (template, rules)
}

const ATOM_LENGTH: usize = ('Z' as u8 - 'A' as u8) as usize + 1;
const POLY_LINKS: usize = ATOM_LENGTH * ATOM_LENGTH;

#[derive(Debug)]
struct Atom(u16);

impl Atom {
    fn from(c: char) -> Self {
        Self((c as u8 - 'A' as u8) as u16)
    }

    fn pair(c1: char, c2: char) -> Self {
        Self(Self::from(c1).0 + Self::from(c2).0 * ATOM_LENGTH as u16)
    }
}

#[derive(Debug)]
struct PolyState {
    pairs: SVector<u64, POLY_LINKS>,
    last: Atom,
}

impl PolyState {
    fn init(data: Vec<char>) -> Self {
        let mut result = Self {
            pairs: SVector::zeros(),
            last: Atom::from(*data.last().unwrap()),
        };

        for value in (0..data.len() - 1).map(|i| Atom::pair(data[i], data[i + 1]).0) {
            result.pairs[value as usize] += 1;
        }

        result
    }

    fn to(&self) -> SVector<u64, ATOM_LENGTH> {
        let mut result = SVector::zeros();

        for (i, v) in self.pairs.iter().enumerate() {
            result[i % ATOM_LENGTH] += v;
        }
        result[self.last.0 as usize] += 1;

        result
    }
}

#[derive(Debug)]
struct PolyRules {
    map: SMatrix<u64, POLY_LINKS, POLY_LINKS>,
}

impl PolyRules {
    fn init(data: HashMap<[char; 2], char>) -> Self {
        let mut result = Self { map: SMatrix::zeros() };

        for ([a, b], c) in data {
            let input = Atom::pair(a, b).0 as usize;
            let output = (Atom::pair(a, c).0 as usize, Atom::pair(c, b).0 as usize);

            result.map[(output.0, input)] += 1;
            result.map[(output.1, input)] += 1;
        }

        result
    }
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args().nth(1).expect("Filepath for polymer not provided");
    let rounds = std::env::args()
        .nth(2)
        .expect("Specify how many rounds")
        .parse::<usize>()
        .unwrap();

    let (poly, rules) = poly_parse(&filepath);
    let mut poly = PolyState::init(poly);
    let rules = PolyRules::init(rules);

    println!("Start: {:?}", poly.to());
    for r in 1..=rounds {
        poly.pairs = rules.map * poly.pairs;
        println!("Round {}: {:?}", r, poly.to());
    }

    let final_result = poly.to().iter().filter(|v| **v != 0).map(|v| *v).collect::<Vec<u64>>();
    let (final_max, final_min) = (final_result.iter().max().unwrap(), final_result.iter().min().unwrap());
    println!("Result Max: {:?}", final_max);
    println!("Result Min: {:?}", final_min);
    println!("Answer: {:?}", final_max - final_min);
}
