use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct SubmarinePath {
    twice: Option<String>,
    path: Vec<String>,
    tunnels: Vec<(String, String)>,
}

impl SubmarinePath {
    fn init(tunnels: Vec<(String, String)>) -> Self {
        Self {
            twice: None,
            path: vec!["start".to_owned()],
            tunnels,
        }
    }

    fn revisit(&self) -> Vec<Self> {
        self.tunnels
            .iter()
            // Join both left and right sides of the tunnel tuple into a single
            // interator
            .map(|(cl, _)| cl)
            .chain(self.tunnels.iter().map(|(_, cr)| cr))
            // Filter by caves that you can only visit once
            .filter(|c| c.to_lowercase().eq(*c) && *c != "start" && *c != "end")
            .unique()
            // Create vector of elements containing a copy of itself,
            // except a cave will be able to be revisited twice
            .map(|c| Self {
                twice: Some(c.clone()),
                path: self.path.clone(),
                tunnels: self.tunnels.clone(),
            })
            .collect()
    }

    fn append(&mut self, next: &str) {
        // Search is over when we receive an end
        if next.eq("end") {
            self.tunnels.clear()
        // Remove all tunnels containing curr and next if cave is lowercase
        } else if let Some(curr) = self.path.last() {
            if curr.to_lowercase().eq(curr) {
                // Except when the cave is stored in 'smalltwice'
                if self.twice.is_some() && self.twice.as_ref().unwrap().eq(curr) {
                    self.twice = None;
                } else {
                    self.tunnels
                        .retain(|(cl, cr)| !(cl.eq(curr) || cr.eq(curr)));
                }
            }
        }

        // Update new current cave (last one)
        self.path.push(next.to_owned());
    }

    fn is_ended(&self) -> bool {
        !self.path.is_empty() && self.path.last().unwrap().eq("end")
    }

    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening file");
        let reader = BufReader::new(file);

        Self::init(
            reader
                .lines()
                .map(|l| {
                    l.unwrap()
                        .split('-')
                        .map(|c| c.to_owned())
                        .collect_tuple()
                        .unwrap()
                })
                .collect(),
        )
    }

    fn step(&self) -> Vec<Self> {
        let mut result = Vec::new();

        for next in self.tunnels.iter().filter_map(|(cl, cr)| {
            let curr = self.path.last().unwrap();
            if curr.eq(cl) {
                Some(cr)
            } else if curr.eq(cr) {
                Some(cl)
            } else {
                None
            }
        }) {
            let mut branch = self.clone();
            branch.append(next);
            result.push(branch);
        }

        result
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for input not provided");

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let mut graphs = vec![SubmarinePath::parse(&filepath)];
    let mut finished = Vec::new();
    while let Some(g) = graphs.pop() {
        if g.is_ended() {
            finished.push(g);
        } else {
            graphs.append(&mut g.step());
        }
    }
    println!("Problem #1: {:?}", finished.len());

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    let mut graphs = SubmarinePath::parse(&filepath).revisit();
    let mut finished = Vec::new();
    while let Some(g) = graphs.pop() {
        if g.is_ended() {
            finished.push(g);
        } else {
            graphs.append(&mut g.step());
        }
    }
    println!(
        "Problem #2: {:?}",
        finished.iter().unique_by(|g| g.path.clone()).count()
    );
}
