pub mod game {

    pub struct Game {
        rolls: Vec<i32>,
        last_roll: i32,
    }

    impl Game {
        pub fn new() -> Game {
            Game {
                rolls: Vec::new(),
                last_roll: 0,
            }
        }

        pub fn add_scores(&mut self, input: String) {
            for (_i, c) in input.char_indices() {
                self.roll_char(c);
            }
        }

        pub fn roll_char(&mut self, c: char) {
            if c == 'X' {
                self.roll(10)
            }

            if c == '/' {
                self.roll(10 - self.last_roll);
            }

            if c == '-' {
                self.roll(0);
            }

            if c.is_numeric() {
                self.roll(c.to_digit(10).unwrap() as i32);
            }
        }

        pub fn roll(&mut self, pins: i32) {
            self.rolls.push(pins);
            self.last_roll = pins;
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
