use std::io;

use game::game::Game;
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut game = Game::new();

    game.score_from_string(input);

    println!("Score: {}", game.score());
}
