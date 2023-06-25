use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

pub use super::reactor::Reactor;

const REACTOR_DIMENSIONS: usize = 3;
type Cuboid = Reactor<REACTOR_DIMENSIONS>;

#[derive(Debug)]
pub struct Reboot {
    pub state: HashMap<Cuboid, isize>,
    pub sequence: Vec<(Cuboid, bool)>,
}

impl Reboot {
    pub fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);
        const REGEX_REBOOT: &str = r"^(on|off) x=(-?\d*)..(-?\d*),y=(-?\d*)..(-?\d*),z=(-?\d*)..(-?\d*)";

        Self {
            state: HashMap::new(),
            sequence: {
                reader
                    .lines()
                    .map(|l| {
                        let l = l.unwrap();
                        let c = Regex::new(REGEX_REBOOT)
                            .unwrap()
                            .captures(&l)
                            .expect("input data does not match regex");

                        (
                            Reactor::init(
                                (0..(REACTOR_DIMENSIONS * 2))
                                    .step_by(2)
                                    .map(|i| {
                                        [
                                            (&c[i + 2]).parse::<_>().unwrap(),
                                            (&c[i + 3]).parse::<_>().unwrap(),
                                        ]
                                    })
                                    .collect_vec()
                                    .try_into()
                                    .unwrap(),
                            ),
                            (&c[1]).to_string() == "on",
                        )
                    })
                    .collect_vec()
                    .into_iter()
                    .rev()
                    .collect()
            },
        }
    }

    pub fn insert(&mut self, reactor: Cuboid, value: isize) {
        let value = *self.state.get(&reactor).unwrap_or(&0) + value;
        if value == 0 {
            self.state.remove(&reactor);
        } else {
            self.state.insert(reactor, value);
        }
    }

    pub fn retain_by_limit(&mut self, value: isize) {
        self.sequence.retain(|(c, _)| c.limit() as isize <= value);
    }

    pub fn process(&mut self) {
        while let Some((command, add)) = self.sequence.pop() {
            let result = self.state.clone();

            for (reactor, value) in result.into_iter() {
                if let Some(common) = reactor.intersect(&command) {
                    self.insert(common, -value);
                }
            }

            if add {
                self.insert(command, 1);
            }
        }
    }

    pub fn n_elems(&self) -> isize {
        self.state.iter().map(|(r, v)| r.n_elems() * *v).sum()
    }
}
