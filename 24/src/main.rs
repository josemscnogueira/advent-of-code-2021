use rand::{rngs::ThreadRng, Rng};
use std::{fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

#[derive(Debug)]
enum Atom {
    W,
    X,
    Y,
    Z,
    Value(i64),
}

impl Atom {
    fn from(c: &str) -> Self {
        match c {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            other => Self::Value(other.parse::<i64>().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Read(Atom),
    Add(Atom, Atom),
    Sub(Atom, Atom),
    Mul(Atom, Atom),
    Div(Atom, Atom),
    Mod(Atom, Atom),
    Eql(Atom, Atom),
}

#[derive(Debug)]
struct Alu {
    instructions: Vec<Operation>,
}

impl Alu {
    fn get_register_index(atom: &Atom) -> usize {
        match atom {
            Atom::W => 0,
            Atom::X => 1,
            Atom::Y => 2,
            Atom::Z => 3,
            _ => panic!("{:?} is not a valid Register representation", atom),
        }
    }

    fn parse(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Error while opening cave file");
        let reader = BufReader::new(file);
        let mut instructions = Vec::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let mut splits = line.split(" ");
            let command = splits.next().unwrap();
            let index = Atom::from(splits.next().unwrap());
            let other = splits.next();
            let other = if let Some(other) = other {
                Ok(Atom::from(other))
            } else {
                Err("Could not parse required argument")
            };

            match command {
                "inp" => {
                    instructions.push(Operation::Read(index));
                }
                "add" => {
                    instructions.push(Operation::Add(index, other.unwrap()));
                }
                "sub" => {
                    instructions.push(Operation::Sub(index, other.unwrap()));
                }
                "mul" => {
                    instructions.push(Operation::Mul(index, other.unwrap()));
                }
                "div" => {
                    instructions.push(Operation::Div(index, other.unwrap()));
                }
                "mod" => {
                    instructions.push(Operation::Mod(index, other.unwrap()));
                }
                "eql" => {
                    instructions.push(Operation::Eql(index, other.unwrap()));
                }
                _ => panic!("Unexpected command: {}", command),
            }
        }

        Self { instructions }
    }

    fn get_atom_value(registers: &[i64], value: &Atom) -> i64 {
        if let Atom::Value(value) = value {
            *value
        } else {
            registers[Self::get_register_index(value)]
        }
    }

    fn is_monad(&mut self, serial: &[i64]) -> i64 {
        let mut registers = [0i64; 4];
        let mut data = serial.into_iter();

        for instruction in &self.instructions {
            match instruction {
                Operation::Read(a) => {
                    registers[Self::get_register_index(a)] =
                        *data.next().unwrap() as i64;
                }
                Operation::Add(a, e) => {
                    registers[Self::get_register_index(a)] +=
                        Self::get_atom_value(&registers, e);
                }
                Operation::Sub(a, e) => {
                    registers[Self::get_register_index(a)] -=
                        Self::get_atom_value(&registers, e);
                }
                Operation::Mul(a, e) => {
                    registers[Self::get_register_index(a)] *=
                        Self::get_atom_value(&registers, e);
                }
                Operation::Div(a, e) => {
                    registers[Self::get_register_index(a)] /=
                        Self::get_atom_value(&registers, e);
                }
                Operation::Mod(a, e) => {
                    registers[Self::get_register_index(a)] %=
                        Self::get_atom_value(&registers, e);
                }
                Operation::Eql(a, e) => {
                    registers[Self::get_register_index(a)] = (registers
                        [Self::get_register_index(a)]
                        == Self::get_atom_value(&registers, e))
                        as i64;
                }
            }
        }

        registers[3]
    }
}

fn randomize_serial(
    digits: &[i64; 14],
    locked: usize,
    request: Option<usize>,
    rng: &mut ThreadRng,
) -> [i64; 14] {
    let mut result = digits.clone();
    let length = digits.len();
    debug_assert!(locked < length);

    let index_length = if let Some(request) = request {
        debug_assert!(request <= (length - locked));
        request
    } else {
        rng.gen_range(1..=(length - locked))
    };

    let index = (1..=index_length)
        .map(|_| rng.gen_range(locked..length))
        .collect_vec();

    index.into_iter().for_each(|i| {
        result[i] = rng.gen_range(1..=9);
    });
    debug_assert!(result.iter().all(|&v| v >= 0 && v <= 9));

    result
}

fn compare_serial(lhs: &[i64; 14], rhs: &[i64; 14]) -> std::cmp::Ordering {
    for i in 0..14 {
        match lhs[i].cmp(&rhs[i]) {
            std::cmp::Ordering::Less => {
                return std::cmp::Ordering::Less;
            }
            std::cmp::Ordering::Greater => {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal => {}
        }
    }
    std::cmp::Ordering::Equal
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");
    let max_tries = std::env::args()
        .nth(2)
        .unwrap_or("1000".to_string())
        .parse::<usize>()
        .unwrap();

    let mut alu = Alu::parse(&filepath);
    let mut digits_locked = [1; 14];
    let mut rng = rand::thread_rng();

    for part in 1..=2 {
        for target in 0..14 {
            for _ in 0..max_tries {
                let mut result = i64::MAX;
                let mut digits = randomize_serial(
                    &digits_locked,
                    target,
                    Some(14 - target),
                    &mut rng,
                );

                while result != 0 {
                    let digits_next =
                        randomize_serial(&digits, target, None, &mut rng);

                    let result_next = alu.is_monad(&digits_next);
                    if result_next.abs() < result.abs() {
                        digits = digits_next;
                        result = result_next;
                    }
                }

                let digit_target = if part == 1 { 9 } else { 1 };
                let compare_target = if part == 1 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                };

                if compare_serial(&digits, &digits_locked) == compare_target {
                    digits_locked = digits;
                    if digits_locked[target] == digit_target {
                        break;
                    }
                }
            }
        }
        println!("Problem #{}: {:?}", part, digits_locked);
    }
}
