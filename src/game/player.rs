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
    pub in_jail: bool,
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
            in_jail: false,
        }
    }
}

pub struct NewPlayer(pub String);
