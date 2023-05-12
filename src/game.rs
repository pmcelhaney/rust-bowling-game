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
                if self.is_spare(i) {
                    score += self.rolls[i + 2];
                }
                if self.is_strike(i) {
                    score += self.rolls[i + 1] + self.rolls[i + 2];
                }
            }
            score
        }

        fn is_spare(&self, frame_index: usize) -> bool {
            if (frame_index + 1) >= self.rolls.len() {
                return false;
            }
            self.rolls[frame_index] + self.rolls[frame_index + 1] == 10
        }

        fn is_strike(&self, frame_index: usize) -> bool {
            if (frame_index + 2) >= self.rolls.len() - 1 {
                return false;
            }
            self.rolls[frame_index] == 10
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}
