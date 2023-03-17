use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;

fn parse(filepath: &str) -> Vec<i64> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    // Create a new grid to be read from file (0,0) from empty grid
    reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect()
}

fn calculate_differences<T>(data: &Vec<T>) -> Vec<T>
where
    T: Copy + Sub<T>,
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    data[0..data.len() - 1]
        .iter()
        .zip(data[1..data.len()].iter())
        .map(|(&v1, &v2)| v2 - v1)
        .collect()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    // Parse file
    let depths = parse(&filepath);
    println!("Total depths: {}", depths.len());

    // Calculate differences
    // Calculate the number of times the depth has increated
    println!(
        "Part1: {}",
        calculate_differences(&depths)
            .into_iter()
            .filter(|&n| n > 0)
            .count()
    );

    // Calculate sliding window
    let window = (0..depths.len() - 2)
        .map(|i| depths[i] + depths[i + 1] + depths[i + 2])
        .collect::<Vec<_>>();
    println!(
        "Part2: {}",
        calculate_differences(&window)
            .into_iter()
            .filter(|&n| n > 0)
            .count()
    );
}
