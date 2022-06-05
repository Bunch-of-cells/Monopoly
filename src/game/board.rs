use super::property::{Money, Property};

pub type TileNumber = u32;

pub enum Tile {
    Property(Property),
    Jail,
    FreeParking,
    GoToJail,
    Go,
    Chance,
    CommunityChest,
    IncomeTax,
    SuperTax,
}

pub struct Board {
    pub tiles: [Tile; 40],
    pub chance_cards: [SpecialCard; 16],
    pub community_chest_cards: [SpecialCard; 16],
}

impl Board {
    pub const JAIL_TILE: TileNumber = 10;
    pub const GO_TILE: TileNumber = 0;
    pub const TOTAL_TILES: u32 = 40;

    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SpecialCard {
    pub body: &'static str,
    pub actions: &'static [Action],
}

pub enum Action {
    Goto(TileNumber),
    PayToBank(Money),
    PayToPlayers(Money),
    TakeFromBank(Money),
    TakeFromPlayers(Money),
    PayPerHouse(Money),
    PayPerHotel(Money),
    PayPerProperty(Money),
}
