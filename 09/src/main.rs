use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> Grid<u8> {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    let mut result = Grid::new(0, 0);
    reader.lines().for_each(|l| {
        result.push_row(
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        )
    });

    result
}

fn calculate_hotspots<T>(grid: &Grid<T>) -> Vec<(usize, usize, T)>
where
    T: PartialOrd + Copy,
{
    let mut result: Vec<(usize, usize, T)> = Vec::new();

    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if ((r == 0) || (grid[r][c] < grid[r - 1][c]))
                && ((c == 0) || (grid[r][c] < grid[r][c - 1]))
                && ((r == (grid.rows() - 1)) || (grid[r][c] < grid[r + 1][c]))
                && ((c == (grid.cols() - 1)) || (grid[r][c] < grid[r][c + 1]))
            {
                result.push((r, c, grid[r][c]));
            }
        }
    }

    result
}

fn calculate_basins<T>(grid: &Grid<T>, hotspots: &Vec<(usize, usize, T)>) -> Vec<usize>
where
    T: PartialOrd + From<u8>,
{
    let mut result = vec![0; hotspots.len()];
    let mut visited: Grid<Option<usize>> = Grid::init(grid.rows(), grid.cols(), None);

    for (index, (r, c, _)) in hotspots.iter().enumerate() {
        let mut queue: Vec<(usize, usize)> = vec![(*r, *c)];

        while let Some((r, c)) = queue.pop() {
            if visited[r][c].is_none() && grid[r][c] < T::from(9u8) {
                result[index] += 1;
                visited[r][c] = Some(index);

                if r > 0 {
                    queue.push((r - 1, c))
                }

                if r < (grid.rows() - 1) {
                    queue.push((r + 1, c));
                }

                if c > 0 {
                    queue.push((r, c - 1))
                }

                if c < (grid.cols() - 1) {
                    queue.push((r, c + 1));
                }
            }
        }
    }

    result
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    // Parse input
    let data = parse(&filepath);
    let hotspots = calculate_hotspots(&data);

    // Problem #1
    println!(
        "Problem #1: {} (size of hotspots = {})",
        hotspots.iter().map(|(_, _, h)| *h as u64 + 1).sum::<u64>(),
        hotspots.len()
    );

    let mut basins = calculate_basins(&data, &hotspots);
    basins.sort();
    basins.reverse();
    println!(
        "Problem #2: {} x {} x {} = {}",
        &basins[0],
        &basins[1],
        &basins[2],
        &basins[0..3].iter().copied().reduce(|a, b| a * b).unwrap()
    );
}
