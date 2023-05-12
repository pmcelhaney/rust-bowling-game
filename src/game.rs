pub mod game {

    pub struct Game {
        rolls: Vec<i32>,
    }

    impl Game {
        pub fn new() -> Game {
            Game { rolls: Vec::new() }
        }

        pub fn roll(&mut self, pins: i32) {
            self.rolls.push(pins);
        }

        pub fn score(&self) -> i32 {
            let mut score = 0;
            for i in 0..self.rolls.len() {
                score += self.rolls[i];
            }
            score
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}
