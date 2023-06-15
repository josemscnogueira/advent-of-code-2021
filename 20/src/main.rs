use grid::Grid;
use itertools::Itertools;
use num::Integer;

use std::{fs::File, io::BufRead, io::BufReader, ops::BitOr};

pub fn decoder_to_string(data: &[bool]) -> String {
    data.iter().map(|c| if *c { '#' } else { '.' }).collect()
}

pub fn image_to_string(data: &Grid<bool>) -> String {
    (0..data.rows())
        .map(|r| decoder_to_string(data.iter_row(r).as_slice()))
        .join("\n")
}

pub fn image_enchance(
    mut input: Grid<bool>,
    decoder: &[bool],
    times: usize,
) -> Grid<bool> {
    debug_assert!(times > 0);

    const HALF_SIZE_KERNEL: isize = 1;
    const FULL_SIZE_KERNEL: usize = (HALF_SIZE_KERNEL as usize * 2 + 1).pow(2);

    let margin = HALF_SIZE_KERNEL as usize * times;
    let rows = input.rows() + 2 * margin;
    let cols = input.cols() + 2 * margin;

    let mut result = Grid::init(rows, cols, false);

    (0..input.rows())
        .cartesian_product(0..input.cols())
        .for_each(|(r, c)| result[r + margin][c + margin] = input[r][c]);

    input = Grid::new(rows, cols);

    for t in 0..times {
        std::mem::swap(&mut result, &mut input);
        result.fill(false);

        for r in 0..rows as isize {
            for c in 0..cols as isize {
                let value = (-HALF_SIZE_KERNEL..=HALF_SIZE_KERNEL)
                    .cartesian_product(-HALF_SIZE_KERNEL..=HALF_SIZE_KERNEL)
                    .map(|(dr, dc)| {
                        let r = r + dr;
                        let c = c + dc;

                        if r >= 0
                            && r < rows as isize
                            && c >= 0
                            && c < cols as isize
                        {
                            input[r as usize][c as usize]
                        } else if t.is_even() {
                            *decoder.last().unwrap()
                                && *decoder.first().unwrap()
                        } else {
                            *decoder.first().unwrap()
                        }
                    })
                    .enumerate()
                    .map(|(i, v)| (v as u16) << (FULL_SIZE_KERNEL - i - 1))
                    .reduce(|acc, e| acc.bitor(e))
                    .unwrap() as usize;

                result[r as usize][c as usize] = decoder[value];
            }
        }
    }

    result
}

pub fn parse(filepath: &str) -> (Vec<bool>, Grid<bool>) {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    let mut decoder = Vec::new();
    let mut image = Grid::new(0, 0);
    let mut is_image = false;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            is_image = true;
        } else {
            let data = line
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unexpected input char = {}", c),
                })
                .collect();

            if is_image {
                image.push_row(data);
            } else {
                decoder = data;
            }
        }
    }

    (decoder, image)
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let (decoder, image) = parse(&filepath);

    let image1 = image_enchance(image.clone(), &decoder, 2);
    println!("Problem #1: {}", image1.iter().filter(|&&v| v).count());

    let image2 = image_enchance(image, &decoder, 50);
    println!("Problem #2: {}", image2.iter().filter(|&&v| v).count());
}
