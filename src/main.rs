use std::io;

use game::game::Game;
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut game = Game::new();

    for (i, c) in input.char_indices() {
        if c == 'X' {
            game.roll(10)
        }

        if c == '/' {
            let last_roll = input.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as i32;
            game.roll(10 - last_roll);
        }

        if c == '-' {
            game.roll(0);
        }

        if c.is_numeric() {
            game.roll(c.to_digit(10).unwrap() as i32);
        }
    }

    println!("Score: {}", game.score());
}
