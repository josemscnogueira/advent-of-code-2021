use priority_queue::DoublePriorityQueue;
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

fn poly_process(input: Vec<char>, rules: &HashMap<[char; 2], char>) -> Vec<char> {
    let length = input.len();
    let mut result = vec![0 as char; length * 2 - 1];

    for index in 0..input.len() - 1 {
        result[index * 2] = input[index];
        result[index * 2 + 1] = *rules.get(&input[index..=index + 1]).unwrap();
    }

    *result.last_mut().unwrap() = *input.last().unwrap();

    result
}

fn poly_count(input: &Vec<char>) -> DoublePriorityQueue<char, u64> {
    let mut result: DoublePriorityQueue<char, u64> = DoublePriorityQueue::new();

    for c in input {
        match result.get(c) {
            None => _ = result.push(*c, 1u64),
            Some((_, p)) => _ = result.change_priority(c, *p + 1),
        }
    }

    result
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args().nth(1).expect("Filepath for polymer not provided");
    let rounds = std::env::args()
        .nth(2)
        .expect("Specify how many rounds")
        .parse::<usize>()
        .unwrap();

    let (mut poly, rules) = poly_parse(&filepath);
    println!("Round 0: length = {}", poly.len());

    for index in 0..rounds {
        poly = poly_process(poly, &rules);
        println!("Round {}: length = {}", index + 1, poly.len());
    }

    let counts = poly_count(&poly);
    println!("Result Max: {:?}", counts.peek_max());
    println!("Result Mix: {:?}", counts.peek_min());
    println!(
        "Answer: {:?}",
        counts.peek_max().unwrap().1 - counts.peek_min().unwrap().1
    );
}
