const MAX_SCORE: u64 = 21;

fn roll_permutations(faces: Vec<u64>, rolls: u64) -> Vec<Vec<u64>> {
    let mut result = Vec::new();

    // Create rolls possibilities in a iterative way
    for r in 0..rolls {
        match r {
            // First roll is special. The number of elements is equal to the
            // number of dice faces
            0 => result = faces.iter().map(|f| vec![*f]).collect(),
            _ => {
                // For each element already produced (in previous iteration)
                // >> Use that element and remove it from the output vector
                for v in std::mem::replace(&mut result, Vec::new()) {
                    // >> For the element previously selected/removedm
                    //    add one new element for each one of the dice faces,
                    //    by concatenating that face to that element
                    for f in &faces {
                        let mut v = v.clone();
                        v.push(*f);
                        result.push(v)
                    }
                }
            }
        }
    }

    // Return result
    result
}

fn roll_stats(roll_sets: Vec<Vec<u64>>) -> Vec<(u64, u64)> {
    let mut result = std::collections::HashMap::new();

    // From all possible roll sets, take each one
    for r in roll_sets {
        // Compute the some of all rolls in that set, given it the final score
        let sum: u64 = r.iter().sum();

        // Count the number of ocurrences of that role score
        match result.get_mut(&sum) {
            Some(count) => *count = *count + 1,
            None => _ = result.insert(sum, 1),
        }
    }

    // Return all (score, counts) as a vector
    result.iter().map(|(k, v)| (*k, *v)).collect()
}

#[derive(Clone, Copy)]
struct Player {
    score: u64,
    position: u64,
}

#[derive(Clone, Copy)]
struct Universe {
    p1: Player,   // Player one state
    p2: Player,   // Player two state
    select: bool, // Select which player will roll the dice: (false -> p1, true -> p2)
    count: u64,   // Encodes representativity of this universe (since several game states can lead to this outcome)
}

fn roll_player(multiverse: &mut Vec<Universe>, stats: &Vec<(u64, u64)>) -> Option<(u64, u64)> {
    // Result containg the number of wins in this round for each player
    let mut result = (0, 0);

    // Obtain one universe per player
    match multiverse.pop() {
        // If there are universes to be considered
        Some(universe) => {
            // For each one of the possible outcomes, after all dice attemps
            // have been rolled
            for (roll_value, roll_counter) in stats {
                // We will create one universe for each one of these possibilities,
                // where each possibility happened a 'roll_counter' amount of times
                let mut spawn = universe;

                // >> Calculate new position and score for the playing player
                if spawn.select {
                    spawn.p2.position = (spawn.p2.position + roll_value) % 10;
                    spawn.p2.score += spawn.p2.position + 1;
                } else {
                    spawn.p1.position = (spawn.p1.position + roll_value) % 10;
                    spawn.p1.score += spawn.p1.position + 1;
                }

                // Update spawn representativity in the multiverse
                spawn.count *= roll_counter;

                // Update player who is playing
                spawn.select = !spawn.select;

                // If player one exceeded the maximum score,
                // update the result but don't store the new spawned universe
                if spawn.p1.score >= MAX_SCORE {
                    result.0 += spawn.count;
                // Same consideration for player two
                } else if spawn.p2.score >= MAX_SCORE {
                    result.1 += spawn.count;
                // Otherwise, simply put the newly spawned universe in the queue
                } else {
                    multiverse.push(spawn);
                }
            }
        }

        _ => return None,
    }

    Some(result)
}

fn main() {
    println!("DiracDice Game start");
    println!("Original player starting positions = (2, 10)");

    // Calculate for this particular example, the statistics about what kind of
    // position displacement a player can get, with a pre-determined number of
    // rolls
    //
    // In this particular case, we have 3-rolls of a three-faced dice (1,2,3)
    // Score, Occurrences (total is 27)
    //     3,           1
    //     4,           3
    //     5,           6
    //     6,           7
    //     7,           6
    //     8,           3
    //     9,           1
    let stats = roll_stats(roll_permutations(vec![1, 2, 3], 3));
    let mut score: (u64, u64) = (0, 0);

    // Create first universe (starting point)
    let mut multiverse = vec![Universe {
        p1: Player {
            score: 0,
            position: std::env::args()
                .nth(1)
                .expect("Provide score for player #1")
                .parse::<u64>()
                .unwrap()
                - 1,
        },
        p2: Player {
            score: 0,
            position: std::env::args()
                .nth(2)
                .expect("Provide score for player #2")
                .parse::<u64>()
                .unwrap()
                - 1,
        },
        select: false,
        count: 1,
    }];

    while let Some(round) = roll_player(&mut multiverse, &stats) {
        score.0 += round.0;
        score.1 += round.1;
    }

    println!("Player 1 won {} times", score.0);
    println!("Player 2 won {} times", score.1);
}
