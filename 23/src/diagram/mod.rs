use grid::Grid;
use itertools::Itertools;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Amphipods {
    A,
    B,
    C,
    D,
}

impl Amphipods {
    pub fn energy_score(&self) -> usize {
        match self {
            Amphipods::A => 1,
            Amphipods::B => 10,
            Amphipods::C => 100,
            Amphipods::D => 1000,
        }
    }

    pub fn target_room(&self) -> usize {
        match self {
            Amphipods::A => 0,
            Amphipods::B => 1,
            Amphipods::C => 2,
            Amphipods::D => 3,
        }
    }
}

impl TryFrom<Element> for Amphipods {
    type Error = &'static str;

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        match value {
            Element::Hallway(Some(a)) => Ok(a),
            Element::Room(Some(a)) => Ok(a),
            _ => Err("Unable to convert Element to Amphipods"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Element {
    Wall,
    Outside,
    Hallway(Option<Amphipods>),
    Room(Option<Amphipods>),
}

impl Default for Element {
    fn default() -> Self {
        Self::Outside
    }
}

pub type Diagram = Grid<Element>;

pub fn parse(filepath: &str) -> Diagram {
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);

    let mut columns = None;
    let mut result = Grid::new(0, 0);

    for line in reader.lines().into_iter().map(|l| l.unwrap()) {
        let mut elements = line
            .chars()
            .map(|c| match c {
                '#' => Element::Wall,
                ' ' => Element::Outside,
                '.' => Element::Hallway(None),
                'A' => Element::Room(Some(Amphipods::A)),
                'B' => Element::Room(Some(Amphipods::B)),
                'C' => Element::Room(Some(Amphipods::C)),
                'D' => Element::Room(Some(Amphipods::D)),
                _ => panic!("Unexpected character {}", c),
            })
            .collect_vec();

        if columns.is_none() {
            columns = Some(elements.len());
        } else if elements.len() < columns.unwrap() {
            elements.extend(
                (0..(columns.unwrap() - elements.len()))
                    .map(|_| Element::Outside),
            );
        }

        result.push_row(elements);
    }

    result
}
