use grid::Grid;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

#[derive(Debug)]
struct LineSegment {
    start: (usize, usize),
    end: (usize, usize),
    delta: (isize, isize),
}

fn parse(filepath: &str) -> Vec<LineSegment> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    // Create a new grid to be read from file (0,0) from empty grid
    let mut result = Vec::new();
    for l in reader.lines().map(|l| l.unwrap()) {
        let s: (&str, &str) = l.split(" -> ").collect_tuple().unwrap();

        let start: (usize, usize) =
            s.0.split(",")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();
        let end: (usize, usize) =
            s.1.split(",")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();
        let delta = (
            end.0 as isize - start.0 as isize,
            end.1 as isize - start.1 as isize,
        );
        result.push(LineSegment {
            start,
            end,
            delta: (
                delta.0.checked_div(delta.0.abs()).unwrap_or(0),
                delta.1.checked_div(delta.1.abs()).unwrap_or(0),
            ),
        });
    }

    result
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    // Parse one
    let segments = parse(&filepath);
    let max_value = segments
        .iter()
        .map(|l| {
            [l.start.0, l.start.1, l.end.0, l.end.1]
                .into_iter()
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        + 1;
    let mut overlaps = Grid::init(max_value, max_value, 0usize);

    // Solve part1
    for s in segments.iter().filter(|l| l.delta.0 == 0 || l.delta.1 == 0) {
        let mut point = s.start;
        loop {
            overlaps[point.0][point.1] += 1;

            if point == s.end {
                break;
            } else {
                point.0 = point.0.wrapping_add_signed(s.delta.0);
                point.1 = point.1.wrapping_add_signed(s.delta.1);
            }
        }
    }

    println!("Part1: {}", overlaps.iter().filter(|&&c| c > 1).count());

    // Solve part2
    overlaps.fill(0);

    for s in segments {
        let mut point = s.start;
        loop {
            overlaps[point.0][point.1] += 1;

            if point == s.end {
                break;
            } else {
                point.0 = point.0.wrapping_add_signed(s.delta.0);
                point.1 = point.1.wrapping_add_signed(s.delta.1);
            }
        }
    }

    println!("Part2: {}", overlaps.iter().filter(|&&c| c > 1).count());
}
