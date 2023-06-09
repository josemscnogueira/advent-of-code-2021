use core::panic;
use grid::Grid;
use itertools::Itertools;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum OrigamiFold {
    Left(usize),
    Up(usize),
}

impl OrigamiFold {
    fn init(line: &str) -> Self {
        let values = line
            .split("fold along ")
            .last()
            .unwrap()
            .split("=")
            .collect_tuple::<(&str, &str)>()
            .unwrap();

        match values.0 {
            "x" => Self::Left(values.1.parse().unwrap()),
            "y" => Self::Up(values.1.parse().unwrap()),
            _ => panic!("Unexpected fold expression {:?}", values),
        }
    }
}

#[derive(Debug)]
struct OrigamiPaper {
    grid: Grid<bool>,
    rows: usize,
    cols: usize,
    folds: Vec<OrigamiFold>,
}

impl fmt::Display for OrigamiPaper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            writeln!(
                f,
                "{:02}: {}",
                r,
                (0..self.cols)
                    .map(|c| if self.grid[r][c] { "#" } else { "." })
                    .join(""),
            )?
        }
        Ok(())
    }
}

impl OrigamiPaper {
    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening file");
        let reader = BufReader::new(file);

        let mut dots = Vec::new();
        let mut folds = Vec::new();
        let mut x_max = 0usize;
        let mut y_max = 0usize;

        for l in reader.lines().map(|l| l.unwrap()) {
            if !l.is_empty() {
                if l.starts_with("fold") {
                    folds.push(OrigamiFold::init(&l));
                } else {
                    let (x, y) = l
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap();

                    x_max = std::cmp::max(x_max, x);
                    y_max = std::cmp::max(y_max, y);
                    dots.push((x, y));
                }
            }
        }

        let mut grid = Grid::init(y_max + 1, x_max + 1, false);
        dots.into_iter().for_each(|(x, y)| grid[y][x] = true);

        Self {
            rows: grid.rows(),
            cols: grid.cols(),
            grid: grid,
            folds: folds.into_iter().rev().collect(),
        }
    }

    fn len(&self) -> usize {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .filter(|(r, c)| self.grid[*r][*c])
            .count()
    }

    fn fold(&mut self) {
        match self.folds.pop().unwrap() {
            OrigamiFold::Left(value) => {
                let range = std::cmp::min(value, self.cols - value - 1);
                self.cols = value;
                for r in 0..self.rows {
                    for c in 1..=range {
                        self.grid[r][value - c] |= self.grid[r][value + c];
                    }
                }
            }
            OrigamiFold::Up(value) => {
                let range = std::cmp::min(value, self.rows - value - 1);
                self.rows = value;
                for r in 1..=range {
                    for c in 0..self.cols {
                        self.grid[value - r][c] |= self.grid[value + r][c];
                    }
                }
            }
        }
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let mut origami = OrigamiPaper::parse(&filepath);

    origami.fold();
    println!("Problem #1: {}", origami.len());

    while origami.folds.len() > 0 {
        origami.fold();
    }
    println!("Problem #2:");
    println!("{}", origami);
}
