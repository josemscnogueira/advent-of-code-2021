use itertools::Itertools;
use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Clone, Copy)]
struct Array2D<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct ShotScenario {
    p0: Array2D<i32>,
    target: Array2D<std::ops::Range<i32>>,
}

// Note: Trickshot used to have more data
#[derive(Debug)]
struct TrickShot {
    peak: i32,
}

impl TrickShot {
    #[allow(dead_code)]
    fn init() -> Self {
        Self { peak: i32::MIN }
    }

    fn push(&mut self, position: Array2D<i32>) {
        self.peak = std::cmp::max(self.peak, position.y);
    }
}

impl ShotScenario {
    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);
        let data = &reader.lines().next().unwrap().unwrap();
        let captures =
            Regex::new(r"^target area: x=(-?\d*)..(-?\d*), y=(-?\d*)..(-?\d*)")
                .unwrap()
                .captures(data)
                .expect("input data does not match regex");

        Self {
            p0: Array2D { x: 0, y: 0 },
            target: Array2D {
                x: (&captures[1]).parse::<i32>().unwrap()
                    ..(&captures[2]).parse::<i32>().unwrap() + 1,
                y: (&captures[3]).parse::<i32>().unwrap()
                    ..(&captures[4]).parse::<i32>().unwrap() + 1,
            },
        }
    }

    fn is_inside_target(&self, position: Array2D<i32>) -> bool {
        self.target.y.contains(&position.y)
            && self.target.x.contains(&position.x)
    }

    fn shot(&self, mut velocity: Array2D<i32>) -> Option<TrickShot> {
        let mut position = self.p0;
        let mut result = TrickShot::init();

        while !self.is_inside_target(position)
            && (position.y >= self.target.y.start)
        {
            result.push(position);

            position.x += velocity.x;
            position.y += velocity.y;
            velocity.y -= 1;
            velocity.x -= 1 * velocity.x.signum();
        }

        result.push(position);
        if self.is_inside_target(position) {
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    let scenario = ShotScenario::parse(&filepath);

    let vel_y_max = std::cmp::max(
        scenario.target.y.start.abs(),
        scenario.target.y.end.abs(),
    );
    let vel_x_max = std::cmp::max(
        scenario.target.x.start.abs(),
        scenario.target.x.end.abs(),
    );

    let shots = (0..=vel_x_max)
        .cartesian_product(-vel_y_max..=vel_y_max)
        .filter_map(|(x, y)| scenario.shot(Array2D { x, y }))
        .collect_vec();

    println!(
        "Problem #1: {}",
        shots
            .iter()
            .sorted_by_key(|s| s.peak)
            .rev()
            .nth(0)
            .unwrap()
            .peak
    );
    println!("Problem #2: {:?}", shots.len());
}
