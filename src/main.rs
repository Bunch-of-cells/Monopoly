use monopoly::*;

fn main() {
    let _game = Game::new()
        .player(NewPlayer("Player 1".to_string()))
        .player(NewPlayer("Player 2".to_string()));
}
