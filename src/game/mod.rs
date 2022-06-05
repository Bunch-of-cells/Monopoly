pub mod board;
pub mod player;
pub mod property;

use board::{Board, Tile};
use player::{NewPlayer, Player, PlayerId};
use property::Money;
use rand::Rng;

pub struct Game {
    players: Vec<Player>,
    board: Board,
    turn: Option<PlayerId>,
}

impl Game {
    pub const GO: Money = Money(200);
    pub const SUPER_TAX: Money = Money(100);
    pub const INCOME_TAX: Money = Money(200);

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

    pub fn step(&mut self) {
        let player_id = self.turn.expect("No player in the game");
        let player = self.players.iter_mut().find(|p| p.id == player_id).unwrap();
        let mut dice = rand::thread_rng();
        let dice_roll = dice.gen_range(1..=6);
        let old = player.tile;
        player.tile = (player.tile + dice_roll) % Board::TOTAL_TILES;
        if player.tile < old {
            player.money += Self::GO;
        }
        match &self.board.tiles[player.tile as usize] {
            Tile::Property(p) => {
                if let Some(rent) = p.rent(self) {
                    self.players
                        .iter_mut()
                        .find(|p| p.id == player_id)
                        .unwrap()
                        .money -= rent;
                }
            }
            Tile::GoToJail => {
                player.tile = Board::JAIL_TILE;
                player.in_jail = true;
            },
            Tile::Chance => todo!(),
            Tile::CommunityChest => todo!(),
            Tile::IncomeTax => player.money -= Self::INCOME_TAX,
            Tile::SuperTax => player.money -= Self::SUPER_TAX,
            Tile::FreeParking | Tile::Go | Tile::Jail => (),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
