use grid::Grid;
use num::Bounded;
use priority_queue::PriorityQueue;
use std::{fs::File, io::BufRead, io::BufReader};

fn grid_parse(filepath: &str) -> Grid<u8> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    // Create a new grid to be read from file (0,0) from empty grid
    let mut results: Grid<u8> = Grid::new(0, 0);

    // Grid will be populated from file
    // >> For each line
    for line in reader.lines() {
        // >> We will have one row
        results.push_row(
            line.unwrap()
                .to_string()
                .chars()
                // >> To be converted to a vector of digits (from 0 to 1)
                //    representing the weight of the path finding graph
                // >> This will fail if the rows don't have the same number of elements
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );
    }

    // Lgging
    println!("Read grid with size = {:?}", results.size());

    // Return grid
    results
}

// TODO: Make this function generic on u8
fn grid_expand(origin: Grid<u8>, factor: usize) -> Grid<u8> {
    // Return input grid if the factor is equal to one
    if factor == 1 {
        return origin;
    }

    // Otherwise, create a new grid with a size determined by 'factor'
    let mut result = Grid::new(origin.rows() * factor, origin.cols() * factor);

    // Cycle all tiles
    for row_tile in 0..factor {
        for col_tile in 0..factor {
            // We have to add +1 each time we go downwards/eastwards a tile
            // Tile (0, 0) is exactly the same as the original map
            let factor_tile = (row_tile + col_tile) as u8;

            // For all values in the current tile
            for r in 0..origin.rows() {
                for c in 0..origin.cols() {
                    // Add the tile factor to the original weight
                    let mut val = origin[r][c] + factor_tile;
                    // If the value exceeds 9, wrap it (9->1)
                    if val > 9 {
                        val = val - 9;
                    }

                    // Keep new value in the map containing all tiles
                    result[row_tile * origin.rows() + r][col_tile * origin.cols() + c] = val;
                }
            }
        }
    }

    // Return result
    result
}

fn calc_priority<T>(value: T) -> T
where
    T: Bounded + std::ops::Sub<T, Output = T>,
{
    T::max_value() - value
}

// TODO: Make generic on u64
fn path_finding_algorithm<T>(weights: &Grid<T>) -> Grid<Option<u64>>
where
    T: Copy,
    u64: From<T>,
{
    // Return if inputs are empty
    let rows = weights.rows();
    let cols = weights.cols();
    if rows == 0 || cols == 0 {
        return Grid::new(0, 0);
    }

    // Create list of distances, where the origin of the graph is the top-left corner
    // and the target of the grapth is the bottom right
    let mut distances: Grid<Option<u64>> = Grid::init(weights.rows(), weights.cols(), None);
    distances[0][0] = Some(0);

    // Create first candidate candidate (we start at the (0,0) vertex)
    let mut candidates = PriorityQueue::new();
    candidates.push((0, 0), calc_priority(distances[0][0].unwrap()));

    // Continue the algorithm until we've runned out of candidates
    let mut counter = 0;
    let mut max_candidates = 0;
    while let Some(((r, c), _)) = candidates.pop() {
        // If we are in the target vertex, stop
        if r == (rows - 1) && c == (cols - 1) {
            candidates.clear();
            break;
        }

        // Check right vicinity
        if c < cols - 1 && distances[r][c + 1].is_none() {
            // Calculate new distance
            let score = u64::from(weights[r][c + 1]) + distances[r][c].unwrap();
            distances[r][c + 1] = Some(score);

            // Put new candidate in the processing queue
            candidates.push((r, c + 1), calc_priority(score));
        }

        // Check lower vicinity
        if r < rows - 1 && distances[r + 1][c].is_none() {
            // Calculate new distance
            let score = u64::from(weights[r + 1][c]) + distances[r][c].unwrap();
            distances[r + 1][c] = Some(score);

            // Put new candidate in the processing queue
            candidates.push((r + 1, c), calc_priority(score));
        }

        // Check left vicinity
        if c > 0 && distances[r][c - 1].is_none() {
            // Calculate new distance
            let score = u64::from(weights[r][c - 1]) + distances[r][c].unwrap();
            distances[r][c - 1] = Some(score);

            // Put new candidate in the processing queue
            candidates.push((r, c - 1), calc_priority(score));
        }

        // Check upper vicinity
        if r > 0 && distances[r - 1][c].is_none() {
            // Calculate new distance
            let score = u64::from(weights[r - 1][c]) + distances[r][c].unwrap();
            distances[r - 1][c] = Some(score);

            // Put new candidate in the processing queue
            candidates.push((r - 1, c), calc_priority(score));
        }

        if candidates.len() > max_candidates {
            max_candidates = candidates.len();
        }

        // Progress print
        counter = counter + 1;
        println!(
            "Progress {:.2} (queue={}/{},processed={})",
            counter as f32 / ((rows * cols) as f32),
            candidates.len(),
            max_candidates,
            counter,
        );
    }

    // Print final result
    println!(
        "Final result in ({},{}) is {:?}",
        rows - 1,
        cols - 1,
        distances[rows - 1][cols - 1]
    );

    // Return the distances map
    distances
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for cave not provided");

    // Parse scale from second argument (optional, default=1)
    let scale: usize = std::env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse()
        .unwrap();

    // Read the path fiding grid, contaning weights for each node to be visited
    let weights = grid_parse(&filepath);

    // Scale map if required by input arguments
    let weights = grid_expand(weights, scale);

    // Create grid of distances
    path_finding_algorithm(&weights);
}
