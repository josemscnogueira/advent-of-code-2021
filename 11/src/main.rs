use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct EnergyGrid {
    state: Grid<u8>,
    length: usize,
    queue: Vec<(usize, usize)>,
    flashes: usize,
    generation: usize,
}

impl EnergyGrid {
    fn init(input: Grid<u8>) -> Self {
        let queue = Vec::with_capacity(input.rows());
        let length = input.rows() * input.cols();
        Self {
            state: input,
            length,
            queue,
            flashes: 0,
            generation: 0,
        }
    }

    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening file");
        let reader = BufReader::new(file);

        let mut state = Grid::new(0, 0);
        reader.lines().for_each(|l| {
            state.push_row(
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            )
        });

        Self::init(state)
    }

    fn increment_octopus(&mut self, row: usize, col: usize) {
        if self.state[row][col] < 10 {
            if self.state[row][col] == 9 {
                self.queue.push((row, col));
            }
            self.state[row][col] += 1;
        }
    }

    fn increase_level(&mut self) -> &mut Self {
        for r in 0..self.state.rows() {
            for c in 0..self.state.cols() {
                self.increment_octopus(r, c);
            }
        }
        self
    }

    fn normalize_level(&mut self) -> &mut Self {
        self.state.iter_mut().filter(|v| **v > 9).for_each(|v| {
            *v = 0;
        });
        self
    }

    fn resolve_flashes(&mut self) -> &mut Self {
        while let Some((r, c)) = self.queue.pop() {
            self.flashes += 1;

            for r in r.saturating_sub(1)..=num::clamp(r + 1, 0, self.state.rows() - 1) {
                for c in c.saturating_sub(1)..=num::clamp(c + 1, 0, self.state.cols() - 1) {
                    self.increment_octopus(r, c);
                }
            }
        }
        self
    }

    fn step(&mut self) -> &mut Self {
        self.generation += 1;
        self.increase_level().resolve_flashes().normalize_level()
    }

    fn syncronize(&mut self) -> &mut Self {
        let mut total_prev = self.flashes;
        while (self.step().flashes - total_prev) < self.length {
            total_prev = self.flashes;
        }
        self
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let mut data = EnergyGrid::parse(&filepath);

    (0..100).for_each(|_| {
        data.step();
    });
    println!("Problem #1: {:?}", data.flashes);
    println!("Problem #2: {:?}", data.syncronize().generation);
}
