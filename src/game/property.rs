use super::player::PlayerId;

pub enum CardType {
    UtilCard {
        pair: &'static str,
        pair_rent: Money,
    },
    ColoredCard {
        color: Color,
        house_1: Money,
        house_2: Money,
        house_3: Money,
        house_4: Money,
        hotel: Money,
        house_cost: Money,
    },
}

#[derive(Clone, Copy)]
pub enum Color {
    Brown,
    Blue,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    Cyan,
}

pub struct Card {
    pub cost: Money,
    pub rent: Money,
    pub mortgage_value: Money,
    pub name: &'static str,
    pub card_type: CardType,
}

impl Card {
    pub fn color(&self) -> Option<Color> {
        match &self.card_type {
            CardType::UtilCard { .. } => None,
            CardType::ColoredCard { color, .. } => Some(*color),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Money(pub u32);

pub enum Houses {
    None,
    One,
    Two,
    Three,
    Four,
    Hotel,
}

pub struct Property {
    pub card: Card,
    pub houses: Option<Houses>,
    pub owner: Option<PlayerId>,
    pub mortgaged: bool,
}

impl Property {
    pub fn rent(&self) -> Money {
        match self.card.card_type {
            CardType::ColoredCard {
                house_1,
                house_2,
                house_3,
                house_4,
                hotel,
                ..
            } => {
                let rent = match self.houses {
                    None | Some(Houses::None) => self.card.rent,
                    Some(Houses::One) => house_1,
                    Some(Houses::Two) => house_2,
                    Some(Houses::Three) => house_3,
                    Some(Houses::Four) => house_4,
                    Some(Houses::Hotel) => hotel,
                };
                rent
            }
            CardType::UtilCard { .. } => self.card.rent,
        }
    }
}
