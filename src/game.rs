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

            let mut i: usize = 0;

            for _frame_index in 0..10 {
                if self.is_strike(i) {
                    score += self.strike_score(i);
                    i += 1;
                } else if self.is_spare(i) {
                    score += self.spare_score(i);
                    i += 2;
                } else {
                    score += self.sum_of_balls_in_frame(i);
                    i += 2;
                }
            }
            score
        }

        fn is_strike(&self, roll_index: usize) -> bool {
            self.rolls[roll_index] == 10
        }

        fn is_spare(&self, roll_index: usize) -> bool {
            self.rolls[roll_index] + self.rolls[roll_index + 1] == 10
        }

        fn strike_score(&self, roll_index: usize) -> i32 {
            10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2]
        }

        fn spare_score(&self, roll_index: usize) -> i32 {
            10 + self.rolls[roll_index + 2]
        }

        fn sum_of_balls_in_frame(&self, roll_index: usize) -> i32 {
            self.rolls[roll_index] + self.rolls[roll_index + 1]
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}
