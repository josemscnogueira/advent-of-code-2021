use grid::Grid;
use itertools::Itertools;
use nalgebra::{Matrix3, Matrix3xX, Vector3};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::utils::rot90;

#[derive(Debug)]
pub struct ScannerLink {
    pub anchor: usize,
    pub position: Vector3<i32>,
    pub rotation: Matrix3<i32>,
}

#[derive(Debug)]
pub struct Scanner {
    pub id: usize,
    pub beacons: Matrix3xX<i32>,
    pub link: Vec<ScannerLink>,
}

impl Scanner {
    pub const LINK_THRESHOLD: usize = 12;

    pub fn is_absolute(&self) -> bool {
        (self.link.len() == 1) && (self.link[0].anchor == 0)
    }

    pub fn absolute_link(&self) -> Option<&ScannerLink> {
        if self.is_absolute() {
            Some(self.link.first().unwrap())
        } else {
            None
        }
    }

    pub fn push_link(
        &mut self,
        anchor: usize,
        position: Vector3<i32>,
        rotation: Matrix3<i32>,
    ) {
        self.link.push(ScannerLink {
            anchor,
            position,
            rotation,
        });
    }
}

impl Scanner {
    pub fn parse(filepath: &str) -> Vec<Self> {
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);

        let mut result = Vec::new();
        let mut current = Vec::<Vector3<i32>>::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            // If line is emtpy, it's the end of the current scanner
            if line.is_empty() {
                if !current.is_empty() {
                    result.push(Matrix3xX::from_columns(&current));
                }
            // If the line starts with scanner, we should be parsing a new
            // scanner
            } else if line.starts_with("--- scanner") {
                current = Vec::new();
            // Otherwise, we append a new beacon to that specific scanner
            } else {
                current.push(Vector3::from_iterator(
                    line.split(",").map(|l| l.parse().unwrap()),
                ));
            }
        }

        // "for" loop might have ended without us pushing the fianl result to the
        // results
        if !current.is_empty() {
            result.push(Matrix3xX::from_columns(&current));
        }

        // result the results
        result
            .into_iter()
            .enumerate()
            .map(|(i, e)| Self {
                id: i,
                beacons: e,
                link: if i == 0 {
                    vec![ScannerLink {
                        anchor: 0,
                        position: Vector3::zeros(),
                        rotation: Matrix3::identity(),
                    }]
                } else {
                    Vec::new()
                },
            })
            .collect_vec()
    }

    pub fn correlation(&self, beacons: &Matrix3xX<i32>) -> Grid<Vector3<i32>> {
        assert_eq!(self.beacons.nrows(), beacons.nrows());

        let mut result = Grid::new(self.beacons.ncols(), beacons.ncols());

        for bself in 0..result.rows() {
            for bother in 0..result.cols() {
                result[bself][bother] =
                    self.beacons.column(bself) - beacons.column(bother);
            }
        }

        result
    }

    pub fn correlation_peak(
        &self,
        beacons: &Matrix3xX<i32>,
    ) -> (usize, Vector3<i32>) {
        let correlation = self.correlation(beacons);

        let mut peak_vector = None;
        let mut peak_counter = 0;

        for r in 0..correlation.rows() {
            for c in 0..correlation.cols() {
                let candidate = correlation[r][c];
                let counter =
                    correlation.iter().filter(|v| **v == candidate).count();

                if counter > peak_counter {
                    peak_counter = counter;
                    peak_vector = Some(candidate);
                }
            }
        }

        (peak_counter, peak_vector.unwrap_or_default())
    }

    #[allow(dead_code)]
    pub fn get_beacon_combos<'a>(
        &'a self,
        number: usize,
    ) -> impl Iterator<Item = Matrix3xX<i32>> + 'a {
        debug_assert!(number < self.beacons.len());

        (0..self.beacons.ncols()).combinations(number).map(|c| {
            let mut current = Vec::<Vector3<i32>>::with_capacity(c.len());

            c.into_iter().for_each(|i| {
                current.push(Vector3::from_iterator(
                    self.beacons.column(i).iter().map(|v| *v),
                ))
            });

            Matrix3xX::<i32>::from_columns(&current)
        })
    }
}

pub fn generate_links(scanners: &mut [Scanner]) {
    let rotations = rot90();

    for index in 0..scanners.len() {
        let (sref, scanditates) = scanners.split_at_mut(index + 1);
        let sref = &mut sref[index];

        for stgt in scanditates {
            for rmatrix in &rotations {
                let (score, tvector) =
                    sref.correlation_peak(&(rmatrix * &stgt.beacons));

                if score >= Scanner::LINK_THRESHOLD {
                    let r_inv = rmatrix.transpose();
                    sref.push_link(stgt.id, -r_inv * tvector, r_inv);
                    stgt.push_link(sref.id, tvector, rmatrix.clone());
                    break;
                }
            }
        }
    }
}

pub fn normalize_links(scanners: &mut [Scanner]) -> bool {
    let mut normalized_prev: usize = 0;

    loop {
        let mut normalized: usize = 0;

        for index in 0..scanners.len() {
            // Split scanners  into:
            // * target to normalize (scanner.id=0)
            // * potential references to the left of target scanner
            // * potential references to the right of target scanner
            //
            // A scanner reference is any scanner that has a direct link
            // to scanner.id=0
            let (sref_left, stgt) = scanners.split_at_mut(index);
            let (stgt, sref_right) = stgt.split_at_mut(1);
            let stgt = &mut stgt[0];

            // If target scanner has link to scanner.id == 0, then it's
            // considered normalize. Additionally, remove all other
            // (unnecessary links)
            if stgt.link.first().unwrap().anchor == 0 {
                if stgt.link.len() != 1 {
                    stgt.link.retain(|l| l.anchor == 0);
                    debug_assert!(stgt.is_absolute());
                }
                normalized += 1;
            } else {
                // Let's try to find a scanner in our current links which is
                // also a direct link to scanner.id == 0
                let mut link_ref = None;

                for link in stgt.link.iter() {
                    if link.anchor < index {
                        link_ref = sref_left[link.anchor].absolute_link();
                    } else if link.anchor > index {
                        link_ref =
                            sref_right[link.anchor - index - 1].absolute_link();
                    }

                    // If absolute link was found (link to scanner.id==0)
                    // Serch is over
                    if let Some(link_ref) = link_ref {
                        stgt.link = vec![ScannerLink {
                            anchor: link_ref.anchor,
                            position: link_ref.position
                                + link_ref.rotation * link.position,
                            rotation: link_ref.rotation * link.rotation,
                        }];
                        break;
                    }
                }
            }
        }

        if normalized == normalized_prev {
            return false;
        } else if normalized == scanners.len() {
            return true;
        } else {
            normalized_prev = normalized;
        }
    }
}

pub fn join_beacons(input: &[Scanner]) -> Vec<Vector3<i32>> {
    let mut result =
        Vec::with_capacity(input.iter().map(|s| s.beacons.len()).sum());

    for scanner in input {
        let link = scanner.absolute_link().unwrap();

        for beacon in scanner.beacons.column_iter() {
            result.push(link.rotation * beacon + link.position);
        }
    }

    result.into_iter().unique().collect()
}

pub fn max_manhattan_distance(input: &[Scanner]) -> Option<i32> {
    input
        .iter()
        .cartesian_product(input.iter())
        .map(|(s1, s2)| {
            (s1.absolute_link().unwrap().position
                - s2.absolute_link().unwrap().position)
                .into_iter()
                .map(|v| v.abs())
                .sum::<i32>()
        })
        .max()
}
