use itertools::iproduct;
use sorted_vec::SortedVec;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum SnailFishChild {
    Left,
    Right,
}

#[derive(Debug, Eq, Clone)]
struct SnailFishNode<T> {
    path: Vec<SnailFishChild>,
    value: T,
}

impl<T> PartialEq for SnailFishNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<T> Ord for SnailFishNode<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.path.iter().zip(other.path.iter()) {
            if a != b {
                return match a {
                    SnailFishChild::Left => Ordering::Less,
                    SnailFishChild::Right => Ordering::Greater,
                };
            }
        }

        // Ordering depends on the size between
        if self.path.len() == other.path.len() {
            Ordering::Equal
        } else if self.path.len() > other.path.len() {
            match self.path.last().unwrap() {
                SnailFishChild::Left => Ordering::Less,
                SnailFishChild::Right => Ordering::Greater,
            }
        } else {
            match other.path.last().unwrap() {
                SnailFishChild::Left => Ordering::Greater,
                SnailFishChild::Right => Ordering::Less,
            }
        }
    }
}

impl<T> PartialOrd for SnailFishNode<T>
where
    T: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct SnailFishTree {
    data: SortedVec<SnailFishNode<u64>>,
}

impl SnailFishTree {
    fn add(mut lhs: Self, mut rhs: Self) -> Self {
        let mut data = SortedVec::new();

        // Move all tree nodes to new tree, but extend path with
        // SnailFishChild::Left
        while let Some(node) = lhs.data.pop() {
            data.push(SnailFishNode {
                path: vec![SnailFishChild::Left]
                    .into_iter()
                    .chain(node.path.into_iter())
                    .collect(),
                value: node.value,
            });
        }

        // Move all tree nodes to new tree, but extend path with
        // SnailFishChild::Right
        while let Some(node) = rhs.data.pop() {
            data.push(SnailFishNode {
                path: vec![SnailFishChild::Right]
                    .into_iter()
                    .chain(node.path.into_iter())
                    .collect(),
                value: node.value,
            });
        }

        // Return tree
        Self { data }
    }

    fn is_in_regular_pair(&self, key: &[SnailFishChild]) -> bool {
        let key_p = &key[0..key.len() - 1];
        let key_l = [&key_p[..], &[SnailFishChild::Left][..]].concat();
        let key_r = [&key_p[..], &[SnailFishChild::Right][..]].concat();

        if let (Some(_), Some(_)) = (
            self.data.iter().position(|n| *n.path == key_l),
            self.data.iter().position(|n| *n.path == key_r),
        ) {
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        // Cycle all leaves (left->right)
        //   >> this is guaranteed by the usage of SortedVec
        //   >> Split left-most leaf with value bigger than 9
        if let Some(index) = self.data.iter().position(|n| n.value > 9) {
            // Remove element to be split and
            // Pre-compute node values
            let node = self.data.remove_index(index);
            let val_l = node.value / 2;
            let val_r = node.value - val_l;

            // Push new element to the left with floor(value / 2)
            self.data.push(SnailFishNode {
                path: [&node.path[..], &[SnailFishChild::Left][..]].concat(),
                value: val_l,
            });

            self.data.push(SnailFishNode {
                path: [&node.path[..], &[SnailFishChild::Right][..]].concat(),
                value: val_r,
            });

            // Return that something has changed
            true
        } else {
            // ... else, nothing has changed in the tree
            false
        }
    }

    fn explode(&mut self) -> bool {
        // Cycle all leaves (left->right)
        //   >> this is guaranteed by the usage of SortedVec
        //   >> Split left-most leaf-regular-pair with depth >= 4
        if let Some(idx_l) = self.data.iter().position(|n| {
            n.path.len() > 4
                && *n.path.last().unwrap() == SnailFishChild::Left
                && self.is_in_regular_pair(&n.path)
        }) {
            // Pre-fetch values before operations in he sorted vec
            let idx_r = idx_l + 1;
            let key_l = &self.data[idx_l].path;
            let key_p = key_l[0..key_l.len() - 1].to_vec();
            let val_l = self.data[idx_l].value;
            let val_r = self.data[idx_r].value;

            // "Explode" by adding left-leaf value to the next left-side leaf
            if idx_l > 0 {
                let old = self.data.remove_index(idx_l - 1);
                self.data.push(SnailFishNode {
                    path: old.path,
                    value: old.value + val_l,
                });
            }

            // "Explode" by adding right-leaf value to the next right-side leaf
            if idx_r < self.data.len() - 1 {
                let old = self.data.remove_index(idx_r + 1);
                self.data.push(SnailFishNode {
                    path: old.path,
                    value: old.value + val_r,
                });
            }

            // "Explode", eliminate the exploding leaves leaves
            self.data.remove_index(idx_l);
            self.data.remove_index(idx_l); // We can explode again because idx_l + 1 == idx_r

            // Add new node as exploded
            self.data.push(SnailFishNode {
                path: key_p,
                value: 0,
            });

            // Return right away,
            // ... otherwise, continue to search for other pairs
            return true;
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            // Keep exploding and splitting until there's nothing to do
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn magnitude(&self) -> u64 {
        let mut result = self.clone();

        while result.data.len() != 1 {
            // Cycle all leaves (left->right)
            //   >> this is guaranteed by the usage of SortedVec
            //   >> Split left-most leaf-regular-pair with depth >= 4
            if let Some(idx_l) = result.data.iter().position(|n| {
                *n.path.last().unwrap() == SnailFishChild::Left
                    && result.is_in_regular_pair(&n.path)
            }) {
                // "Explode", eliminate the exploding leaves leaves
                let key_l = &result.data[idx_l].path;
                let key_p = key_l[0..key_l.len() - 1].to_vec();
                let val_l = result.data.remove_index(idx_l).value;
                let val_r = result.data.remove_index(idx_l).value;

                // Add new node as exploded
                result.data.push(SnailFishNode {
                    path: key_p,
                    value: val_l * 3 + val_r * 2,
                });
            }
        }

        result.data.last().unwrap().value
    }

    fn from(line: &str, parent: Vec<SnailFishChild>) -> Self {
        let mut level = 0;
        for (i, c) in line.chars().enumerate() {
            match c {
                '[' => level += 1,
                ']' => level -= 1,
                ',' => {
                    if level == 1 {
                        return SnailFishTree::add(
                            SnailFishTree::from(
                                &line[1..i],
                                [&parent[..], &[SnailFishChild::Left][..]]
                                    .concat(),
                            ),
                            SnailFishTree::from(
                                &line[i + 1..line.len() - 1],
                                [&parent[..], &[SnailFishChild::Right][..]]
                                    .concat(),
                            ),
                        );
                    }
                }
                _ => (),
            }
        }

        assert_eq!(level, 0);
        SnailFishTree {
            data: SortedVec::from_unsorted(vec![SnailFishNode {
                path: Vec::new(),
                value: line.parse().unwrap(),
            }]),
        }
    }

    fn to_string(&self) -> String {
        let mut result = SortedVec::from_unsorted(
            self.data
                .iter()
                .map(|n| SnailFishNode {
                    path: n.path.clone(),
                    value: n.value.to_string(),
                })
                .collect(),
        );

        while result.len() != 1 {
            if let Some(idx_l) = result[0..result.len() - 1]
                .iter()
                .zip(&result[1..result.len()])
                .position(|(nl, nr)| {
                    *nl.path.last().unwrap() == SnailFishChild::Left
                        && *nr.path
                            == [
                                &nl.path[0..nl.path.len() - 1][..],
                                &[SnailFishChild::Right][..],
                            ]
                            .concat()
                })
            {
                // "Explode", eliminate the exploding leaves leaves
                let key_l = &result[idx_l].path;
                let key_p = key_l[0..key_l.len() - 1].to_vec();
                let val_l = result.remove_index(idx_l).value;
                let val_r = result.remove_index(idx_l).value; // We can explode again because idx_l + 1 == idx_r

                // Add new node as exploded
                result.push(SnailFishNode {
                    path: key_p,
                    value: format!("[{},{}]", val_l, val_r),
                });
            }
        }

        if result.len() == 1 {
            result.first().unwrap().value.clone()
        } else {
            String::new()
        }
    }

    fn parse_file(filepath: &str) -> Vec<SnailFishTree> {
        // Open a file and read from it
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|s| SnailFishTree::from(&s.unwrap(), Vec::new()))
            .collect()
    }
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for SnailFishNumbers not provided");

    let forest = SnailFishTree::parse_file(&filepath);

    {
        let mut tree = forest.first().unwrap().clone();
        for t in &forest[1..forest.len()] {
            tree = SnailFishTree::add(tree, t.clone());
            tree.reduce();
        }
        println!("Part1: {}", tree.magnitude());
    }
    {
        let result = iproduct!(0..forest.len(), 0..forest.len())
            .map(|(r, c)| {
                let mut tree =
                    SnailFishTree::add(forest[r].clone(), forest[c].clone());
                tree.reduce();

                tree.magnitude()
            })
            .max()
            .unwrap();

        println!("Part2: {}", result);
    }
}
