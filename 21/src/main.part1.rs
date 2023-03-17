const PLAYER_START: [u32; 2] = [2 - 1, 10 - 1];

struct DiceDeterministic {
    counter: u64,
    face: u32,
}

impl DiceDeterministic {
    fn new() -> Self {
        Self {
            counter: 0,
            face: 1,
        }
    }

    fn roll(&mut self) -> u32 {
        let result = self.face;
        self.face = (self.face + 1) % 100;
        self.counter += 1;
        result
    }
}

fn main() {
    println!("DiracDice Game start");

    let mut dice = DiceDeterministic::new();
    let mut select = 0;
    let mut round = 0u64;
    let mut position = PLAYER_START;
    let mut score: [u32; 2] = [0, 0];
    let mut playing = true;

    while playing {
        // Update round index
        round += 1;

        // Roll the dice
        let roll = [dice.roll(), dice.roll(), dice.roll()];

        // Update player position
        position[select] = (position[select] + roll.iter().sum::<u32>()) % 10;

        // Update player score
        score[select] += position[select] + 1;

        // Print
        println!(
            "Round {}: Player {} rolls {}+{}+{} and moves to space {} for a total score of {}",
            round,
            select + 1,
            roll[0],
            roll[1],
            roll[2],
            position[select] + 1,
            score[select]
        );

        if score[select] >= 1000 {
            playing = false;
        }

        // Update player selection
        select = match select {
            1 => 0,
            0 => 1,
            _ => panic!("Select value not possible"),
        }
    }

    println!(
        "Player {} lost with {} points after {} dice rolls. Answer = {}",
        select + 1,
        score[select],
        dice.counter,
        u64::from(score[select]) * dice.counter
    );
}
