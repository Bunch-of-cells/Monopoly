use super::{property::{Property, Money}, board::TileNumber};

#[derive(Clone, Copy)]
pub struct PlayerId(u64);

pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub property: Vec<Property>,
    pub money: Money,
    pub tile: TileNumber,
    pub in_jail: bool,
}
