use itertools::Itertools;

use super::diagram::{Amphipods, Diagram, Element};

#[derive(Debug, Clone)]
pub enum Move {
    Exit((usize, usize), usize),
    Enter(usize, (usize, usize)),
}

#[derive(Debug, Clone)]
pub struct Borrow {
    pub energy: usize,
    pub moves: Vec<Move>,
    hallway: Vec<Option<Amphipods>>,
    rooms: [Vec<Option<Amphipods>>; 4],
}

impl Borrow {
    pub const ENTRANCES: [usize; 4] = [2, 4, 6, 8];
    pub const HALLWAY_SLOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    pub fn parse(filepath: &str) -> Self {
        let diagram = super::diagram::parse(filepath);
        Self::load(diagram)
    }

    pub fn load(diagram: Diagram) -> Self {
        Self::check_diagram(&diagram).expect("Diagram is not valid");

        Self {
            energy: 0,
            moves: Vec::new(),
            hallway: vec![None; 11],
            rooms: [
                vec![
                    Some(diagram[2][3].try_into().unwrap()),
                    Some(diagram[3][3].try_into().unwrap()),
                ],
                vec![
                    Some(diagram[2][5].try_into().unwrap()),
                    Some(diagram[3][5].try_into().unwrap()),
                ],
                vec![
                    Some(diagram[2][7].try_into().unwrap()),
                    Some(diagram[3][7].try_into().unwrap()),
                ],
                vec![
                    Some(diagram[2][9].try_into().unwrap()),
                    Some(diagram[3][9].try_into().unwrap()),
                ],
            ],
        }
    }

    pub fn optimize(start: Self) -> Option<Self> {
        let mut multiverse = vec![start];
        let mut result: Option<Self> = None;

        while let Some(iteration) = multiverse.pop() {
            if result.is_none()
                || (iteration.energy < result.as_ref().unwrap().energy)
            {
                if iteration.is_solved() {
                    result = Some(iteration);
                } else {
                    let possibilities = iteration.get_all_moves();

                    for p in possibilities {
                        let mut iteration_next = iteration.clone();
                        iteration_next.apply_move(p);
                        multiverse.push(iteration_next);
                    }
                }
            }
        }

        result
    }

    pub fn is_solved(&self) -> bool {
        self.hallway.iter().all(|a| a.is_none())
            && self.rooms.iter().enumerate().all(|(index, a)| {
                a.iter().all(|a| {
                    if let Some(a) = a {
                        a.target_room() == index
                    } else {
                        false
                    }
                })
            })
    }

    pub fn apply_move(&mut self, data: Move) {
        match data {
            Move::Enter(slot, (room, level)) => {
                let amphipod = self.hallway[slot].unwrap();
                self.hallway[slot] = None;

                debug_assert!(self.rooms[room][level].is_none());
                self.rooms[room][level] = Some(amphipod);

                self.energy += amphipod.energy_score()
                    * (Self::ENTRANCES[room].abs_diff(slot) + level + 1);
            }
            Move::Exit((room, level), slot) => {
                let amphipod = self.rooms[room][level].unwrap();
                self.rooms[room][level] = None;

                debug_assert!(self.hallway[slot].is_none());
                self.hallway[slot] = Some(amphipod);

                self.energy += amphipod.energy_score()
                    * (Self::ENTRANCES[room].abs_diff(slot) + level + 1);
            }
        }

        self.moves.push(data);
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut result = Vec::new();

        for (index, level) in self.get_movable_amphipods_hallway() {
            let room = self.hallway[index].unwrap().target_room();
            let entrance = Self::ENTRANCES[room];
            let (min, max) = if entrance < index {
                (entrance, index - 1)
            } else {
                (index + 1, entrance)
            };

            // If hallway is clear, that this is a valid move (push)
            if (min..=max).all(|i| self.hallway[i].is_none()) {
                result.push(Move::Enter(index, (room, level)));
            }
        }

        for (room, level) in self.get_movable_amphipods_rooms() {
            for index in Self::HALLWAY_SLOTS {
                let entrance = Self::ENTRANCES[room];
                let (min, max) = if entrance < index {
                    (entrance, index)
                } else {
                    (index, entrance)
                };

                // If hallway is clear, that this is a valid move (push)
                if (min..=max).all(|i| self.hallway[i].is_none()) {
                    result.push(Move::Exit((room, level), index));
                }
            }
        }

        result
    }

    fn get_movable_amphipods_rooms(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for (index, room) in self.rooms.iter().enumerate() {
            for (level, r) in room.iter().enumerate() {
                if let Some(a) = r {
                    if a.target_room() != index
                        || ((level + 1)..room.len())
                            .any(|i| room[i].unwrap().target_room() != index)
                    {
                        result.push((index, level));
                        break;
                    }
                }
            }
        }

        result
    }

    fn get_movable_amphipods_hallway(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for (index, &a) in
            self.hallway.iter().enumerate().filter(|(_, a)| a.is_some())
        {
            let target = a.unwrap().target_room();
            let occupancy = &self.rooms[target];

            for slot in (0..occupancy.len()).rev() {
                if occupancy[slot].is_none() {
                    result.push((index, slot));
                    break;
                } else if occupancy[slot].unwrap().target_room() != target {
                    break;
                }
            }
        }

        result
    }

    pub fn check_diagram(diagram: &Diagram) -> Result<(), &str> {
        let cols = diagram.cols();

        if diagram.rows() != 5 || cols != 13 {
            Err("Diagram dimensions are wrong")
        } else if (0..cols).any(|c| diagram[0][c] != Element::Wall) {
            Err("First row is not all wall")
        } else if (1..(cols - 1))
            .any(|c| diagram[1][c] != Element::Hallway(None))
            || diagram[1].first().unwrap() != &Element::Wall
            || diagram[1].last().unwrap() != &Element::Wall
        {
            Err("Second row does not represent the hallway")
        } else if (0..3)
            .chain((cols - 3)..cols)
            .chain((4..9).step_by(2))
            .any(|c| diagram[2][c] != Element::Wall)
        {
            Err("Third row does not have proper walls around rooms")
        } else if (2..11).step_by(2).any(|c| diagram[3][c] != Element::Wall) {
            Err("Forth row does not have proper walls around rooms")
        } else if (2..11).any(|c| diagram[4][c] != Element::Wall) {
            Err("Fifth row does not have proper walls around rooms")
        } else if [Amphipods::A, Amphipods::B, Amphipods::C, Amphipods::D]
            .into_iter()
            .any(|a| {
                (2..4)
                    .cartesian_product((3..10).step_by(2))
                    .filter(|(r, c)| {
                        if let Element::Room(Some(candidate)) = diagram[*r][*c]
                        {
                            a == candidate
                        } else {
                            false
                        }
                    })
                    .count()
                    != 2
            })
        {
            Err("Rooms don't have two Amphipods of each type")
        } else {
            Ok(())
        }
    }
}
