pub mod board;
pub mod player;
pub mod property;

use board::Board;
use player::{NewPlayer, Player, PlayerId};

pub struct Game {
    players: Vec<Player>,
    board: Board,
    turn: Option<PlayerId>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            board: Board::new(),
            turn: None,
        }
    }

    pub fn player(mut self, player: NewPlayer) -> Game {
        let mut id = rand::random::<u64>();
        while self.players.iter().any(|p| p.id.0 == id) {
            id = rand::random::<u64>();
        }
        let id = PlayerId(id);
        if self.turn.is_none() {
            self.turn = Some(id);
        }
        self.players.push(Player::new(player, id));
        self
    }

    pub fn get_player_by_id(&self, id: PlayerId) -> Option<&Player> {
        self.players.iter().find(|p| p.id == id)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
