use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Reactor<const N: usize> {
    core: [[i32; 2]; N],
}

impl<const N: usize> Reactor<N> {
    pub fn init(core: [[i32; 2]; N]) -> Self {
        Self { core }
    }

    pub fn n_elems(&self) -> isize {
        self.core
            .iter()
            .map(|e| (e[1] + 1 - e[0]) as isize)
            .product()
    }

    pub fn limit(&self) -> i32 {
        self.core
            .iter()
            .map(|i| i[0].abs().max(i[1].abs()))
            .max()
            .unwrap()
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let result = (0..N)
            .map(|d| {
                [
                    self.core[d][0].max(other.core[d][0]),
                    self.core[d][1].min(other.core[d][1]),
                ]
            })
            .collect_vec();

        if result.iter().all(|v| v[0] <= v[1]) {
            Some(Self {
                core: result.try_into().unwrap(),
            })
        } else {
            None
        }
    }
}
