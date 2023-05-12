#[cfg(test)]
mod tests {
    use game::game::Game;

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(1);
        }
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn test_all_twos() {
        let mut game = Game::new();

        for _ in 0..20 {
            game.roll(2);
        }

        assert_eq!(game.score(), 40);
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();

        game.roll(5);
        game.roll(5);
        game.roll(1);

        for _ in 3..20 {
            game.roll(0);
        }

        assert_eq!(game.score(), 11 + 1);
    }
}
