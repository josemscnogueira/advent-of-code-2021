use phf::phf_map;
use std::fs::File;
use std::io::{BufRead, BufReader};

static CHUNK_BOUNDS: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '{' => '}',
    '[' => ']',
    '<' => '>',
};

enum ESyntaxScoring {
    Correct,
    Incomplete(u64),
    Corrupted(u64),
}

impl ESyntaxScoring {
    fn score(c: char) -> Option<u64> {
        match c {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None,
        }
    }

    fn new(line: &Vec<char>) -> Self {
        let mut stack: Vec<char> = Vec::with_capacity(80);

        for c in line {
            match c {
                '(' | '[' | '{' | '<' => stack.push(*c),
                ')' | ']' | '}' | '>' => {
                    let cc = stack.pop().unwrap();
                    if c != CHUNK_BOUNDS.get(&cc).unwrap() {
                        return Self::Corrupted(Self::score(*c).unwrap());
                    }
                }
                _ => panic!("Unexpected character {}", c),
            }
        }

        if stack.is_empty() {
            assert!(false);
            Self::Correct
        } else {
            Self::Incomplete(
                stack
                    .iter()
                    .rev()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unexpected character {}", c),
                    })
                    .reduce(|a, b| a * 5 + b)
                    .unwrap(),
            )
        }
    }
}

fn parse(filepath: &str) -> Vec<Vec<char>> {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    // Parse input
    let data = parse(&filepath);
    let parsed: Vec<ESyntaxScoring> = data.iter().map(|l| ESyntaxScoring::new(l)).collect();

    // Problem #1
    println!(
        "Problem #1: {:?}",
        parsed
            .iter()
            .map(|l| if let ESyntaxScoring::Corrupted(s) = l {
                *s
            } else {
                0
            })
            .sum::<u64>()
    );

    // Problem #2
    let mut scores_incomplete = parsed
        .iter()
        .filter_map(|l| {
            if let ESyntaxScoring::Incomplete(s) = l {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    scores_incomplete.sort_unstable();

    assert!(scores_incomplete.len() % 2 == 1);
    println!(
        "Problem #2: {:?}, size={}/{}",
        scores_incomplete[scores_incomplete.len() / 2],
        scores_incomplete.len(),
        data.len()
    );
}
