use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Board {
    id: usize,
    mask: Grid<bool>,
    numbers: Grid<u8>,
    rows: Vec<u64>,
    cols: Vec<u64>,
    score: Option<u64>,
    round: Option<usize>,
}

impl Board {
    fn from(numbers: Grid<u8>, id: usize) -> Self {
        let rows = (0..numbers.rows())
            .map(|r| numbers.iter_row(r).map(|d| *d as u64).sum())
            .collect();
        let cols = (0..numbers.cols())
            .map(|c| numbers.iter_col(c).map(|d| *d as u64).sum())
            .collect();

        Self {
            id,
            mask: Grid::init(numbers.rows(), numbers.cols(), true),
            numbers,
            rows,
            cols,
            score: None,
            round: None,
        }
    }

    fn compute_score(&self) -> u64 {
        self.numbers
            .iter()
            .zip(self.mask.iter())
            .map(|(d, m)| if *m { *d as u64 } else { 0 })
            .sum()
    }

    fn update(&mut self, value: u8, round: Option<usize>) -> bool {
        // If the board is already complete (=score is valid), just exit
        if let Some(_) = self.score {
            return true;
        }

        // Update board
        let mut bingo = false;
        for r in 0..self.numbers.rows() {
            for c in 0..self.numbers.cols() {
                if self.mask[r][c] && self.numbers[r][c] == value {
                    // Update masks
                    self.mask[r][c] = false;
                    self.rows[r] -= value as u64;
                    self.cols[c] -= value as u64;

                    // If any row or col is complete, just select bingp
                    if self.rows[r] == 0 || self.cols[c] == 0 {
                        bingo = true;
                    }
                }
            }
        }

        // Return bingo value
        if bingo {
            self.score = Some(self.compute_score() * value as u64);
            if round.is_some() {
                self.round = round;
            }
        }
        bingo
    }
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Bingo {
    fn parse(filepath: &str) -> Self {
        // Open a file and read from it
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);

        // Create a new grid to be read from file (0,0) from empty grid
        let mut numbers = Vec::new();
        let mut boards = Vec::new();
        let mut grid: Grid<u8> = Grid::new(0, 0);

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            // Read first line as bingo numbers
            if index == 0 {
                numbers = line
                    .split(",")
                    .into_iter()
                    .map(|d| d.parse::<_>().unwrap())
                    .collect()
            // Start reading bingo boards
            } else {
                // If the line is not empty, add new row to reading bingo grid
                if !line.is_empty() {
                    grid.push_row(
                        line.split(" ")
                            .into_iter()
                            .filter(|d| !d.is_empty())
                            .map(|d| d.parse::<_>().unwrap())
                            .collect(),
                    );
                // If the line is empty, it's time to store the current bingo grid
                } else if grid.rows() > 0 {
                    boards.push(Board::from(
                        std::mem::replace(&mut grid, Grid::new(0, 0)),
                        boards.len(),
                    ));
                }
            }
        }

        // Return bingo
        Self { numbers, boards }
    }
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    // Parse game from file
    let mut game = Bingo::parse(&filepath);

    // Play the game
    for (round, number) in game.numbers.iter().enumerate() {
        let _ = game
            .boards
            .iter_mut()
            .map(|b| b.update(*number, Some(round)))
            .collect::<Vec<bool>>();
    }

    // Print all boards in ascending order of winning place
    game.boards.sort_by_key(|b| b.round.unwrap_or(usize::MAX));
    for b in game.boards {
        println!(
            "Board {} won on round {:?}/{:?} with score {:?}",
            b.id,
            b.round,
            game.numbers.len(),
            b.score
        );
    }
}
