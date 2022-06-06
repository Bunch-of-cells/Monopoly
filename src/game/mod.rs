pub mod board;
pub mod player;
pub mod property;

use board::{Action, Board, Tile};
use player::{NewPlayer, Player, PlayerId};
use property::Money;
use rand::{prelude::SliceRandom, Rng};

pub struct Game {
    players: Vec<Player>,
    board: Board,
    turn: Option<PlayerId>,
}

impl Game {
    pub const GO: Money = Money(200);
    pub const SUPER_TAX: Money = Money(100);
    pub const INCOME_TAX: Money = Money(200);
    pub const JAIL: Money = Money(50);

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

        let dice_roll = rand::thread_rng().gen_range(1..=6);
        let dice_roll2 = rand::thread_rng().gen_range(1..=6);

        if let Some(turns) = player.jail_turns.as_mut() {
            *turns += 1;
            if *turns == 3 {
                player.jail_turns = None;
                player.take(Self::JAIL);
                return;
            } else if dice_roll == dice_roll2 {
                player.jail_turns = None;
            } else if player.get_out_of_jail_by_paying() {
                player.jail_turns = None;
                player.take(Self::JAIL);
                return;
            } else if player.get_out_of_jail_cards > 0 && player.get_out_of_jail_by_card() {
                player.jail_turns = None;
                player.get_out_of_jail_cards -= 1;
                return;
            }
        }

        let old = player.tile;
        player.tile = (player.tile + dice_roll + dice_roll2) % Board::TOTAL_TILES;

        match &self.board.tiles[player.tile as usize] {
            Tile::Chance => {
                let card = self
                    .board
                    .chance_cards
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                println!("{} draws a chance card: {}", player.name, card.body);
                for action in card.actions {
                    match action {
                        Action::Goto(tile) => player.tile = *tile,
                        Action::PayToBank(m) => player.take(*m),
                        Action::PayToPlayers(_) => todo!(),
                        Action::TakeFromBank(m) => player.collect(*m),
                        Action::TakeFromPlayers(_) => todo!(),
                        Action::PayPerHouse(_) => todo!(),
                        Action::PayPerHotel(_) => todo!(),
                        Action::PayPerProperty(_) => todo!(),
                    }
                }
            }
            Tile::CommunityChest => {
                let card = self
                    .board
                    .chance_cards
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                println!(
                    "{} draws a community chest card: {}",
                    player.name, card.body
                );
                for action in card.actions {
                    match action {
                        Action::Goto(tile) => player.tile = *tile,
                        Action::PayToBank(m) => player.take(*m),
                        Action::PayToPlayers(_) => todo!(),
                        Action::TakeFromBank(m) => player.collect(*m),
                        Action::TakeFromPlayers(_) => todo!(),
                        Action::PayPerHouse(_) => todo!(),
                        Action::PayPerHotel(_) => todo!(),
                        Action::PayPerProperty(_) => todo!(),
                    }
                }
            }
            tile => {
                if player.tile < old {
                    player.collect(Self::GO);
                };
                match tile {
                    Tile::IncomeTax => player.take(Self::INCOME_TAX),
                    Tile::SuperTax => player.take(Self::SUPER_TAX),
                    Tile::FreeParking | Tile::Go | Tile::Jail => (),
                    Tile::Property(p) => {
                        if let Some(rent) = p.rent(self) {
                            self.players
                                .iter_mut()
                                .find(|p| p.id == player_id)
                                .unwrap()
                                .take(rent);
                        }
                    }
                    Tile::GoToJail => {
                        player.tile = Board::JAIL_TILE;
                        player.jail_turns = Some(0);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
