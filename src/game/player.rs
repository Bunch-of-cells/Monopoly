use std::cmp::Ordering;

use super::{
    board::{Board, TileNumber},
    property::{Money, Property},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlayerId(pub u64);

pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub property: Vec<Property>,
    pub money: Money,
    pub debt: Money,
    pub tile: TileNumber,
    pub jail_turns: Option<u32>,
    pub get_out_of_jail_cards: u32,
}

impl Player {
    pub fn new(player: NewPlayer, id: PlayerId) -> Self {
        Player {
            id,
            name: player.0,
            property: Vec::new(),
            money: Money(1500),
            debt: Money(0),
            tile: Board::GO_TILE,
            jail_turns: None,
            get_out_of_jail_cards: 0,
        }
    }

    pub fn take(&mut self, money: Money) {
        match (self.money.0.cmp(&money.0), self.money.0.abs_diff(money.0)) {
            (Ordering::Less, x) => {
                self.money = Money(0);
                self.debt += Money(x);
            }
            (Ordering::Equal, _) => {
                self.money = Money(0);
            }
            (Ordering::Greater, x) => {
                self.money = Money(x);
            }
        }
    }

    pub fn collect(&mut self, money: Money) {
        if self.debt.0 >= money.0 {
            self.debt -= money;
        } else {
            self.money += money - self.debt;
            self.debt = Money(0);
        }
    }

    pub fn get_out_of_jail_by_paying(&self) -> bool {
        false
    }

    pub fn get_out_of_jail_by_card(&self) -> bool {
        false
    }
}

pub struct NewPlayer(pub String);
