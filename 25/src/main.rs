use grid::Grid;
use itertools::Itertools;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Default, Clone, Copy)]
enum SeaCucumber {
    #[default]
    None,
    East,
    South,
}

#[derive(Debug)]
struct SeaFloor {
    state: Grid<SeaCucumber>,
}

impl SeaFloor {
    fn step(&mut self) -> bool {
        let east = self.step_east();
        let south = self.step_south();
        east || south
    }

    fn step_south(&mut self) -> bool {
        let rows = self.state.rows();
        let cols = self.state.cols();
        let prev = std::mem::replace(&mut self.state, Grid::new(rows, cols));
        let mut result = false;

        for r in 0..rows {
            for c in 0..cols {
                match prev[r][c] {
                    SeaCucumber::None => (),
                    SeaCucumber::East => self.state[r][c] = SeaCucumber::East,
                    SeaCucumber::South => {
                        let mut rn = if r == (rows - 1) { 0 } else { r + 1 };
                        if let SeaCucumber::None = prev[rn][c] {
                            result = true;
                        } else {
                            rn = r;
                        }
                        self.state[rn][c] = SeaCucumber::South;
                    }
                }
            }
        }
        result
    }

    fn step_east(&mut self) -> bool {
        let rows = self.state.rows();
        let cols = self.state.cols();
        let prev = std::mem::replace(&mut self.state, Grid::new(rows, cols));
        let mut result = false;

        for r in 0..rows {
            for c in 0..cols {
                match prev[r][c] {
                    SeaCucumber::None => (),
                    SeaCucumber::South => self.state[r][c] = SeaCucumber::South,
                    SeaCucumber::East => {
                        let mut cn = if c == (cols - 1) { 0 } else { c + 1 };
                        if let SeaCucumber::None = prev[r][cn] {
                            result = true;
                        } else {
                            cn = c;
                        }
                        self.state[r][cn] = SeaCucumber::East;
                    }
                }
            }
        }
        result
    }

    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);

        let mut state = Grid::new(0, 0);

        reader
            .lines()
            .into_iter()
            .map(|l| l.unwrap())
            .for_each(|l| {
                state.push_row(
                    l.chars()
                        .map(|c| match c {
                            'v' => SeaCucumber::South,
                            '>' => SeaCucumber::East,
                            '.' => SeaCucumber::None,
                            _ => {
                                panic!("SeaCucumber char '{}' is not valid", c)
                            }
                        })
                        .collect_vec(),
                );
            });

        Self { state }
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let mut sea = SeaFloor::parse(&filepath);
    let mut counter = 0usize;
    while sea.step() {
        counter += 1;
    }

    println!("Problem #1: {}", counter + 1);
}
