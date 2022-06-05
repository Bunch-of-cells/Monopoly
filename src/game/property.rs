use std::ops::{Add, Mul, Sub};

use super::{player::PlayerId, Game};

pub enum CardType {
    UtilCard {
        pair: &'static str,
        pair_rent: Money,
    },
    ColoredCard {
        color: Color,
        house_rent: [Money; 5],
        house_cost: Money,
    },
}

#[derive(Clone, Copy, PartialEq)]
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

impl Color {
    pub fn max_cards(&self) -> usize {
        match self {
            Color::Brown
            | Color::Blue
            | Color::Pink
            | Color::Orange
            | Color::Red
            | Color::Yellow => 2,
            Color::Green | Color::Cyan => 3,
        }
    }
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

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        Money(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, other: Money) -> Money {
        Money(self.0 - other.0)
    }
}

impl Mul<u32> for Money {
    type Output = Money;

    fn mul(self, other: u32) -> Money {
        Money(self.0 * other)
    }
}

pub struct Property {
    pub card: Card,
    pub houses: Option<usize>,
    pub owner: Option<PlayerId>,
    pub mortgaged: bool,
}

impl Property {
    pub fn rent(&self, game: &Game) -> Option<Money> {
        if self.mortgaged {
            return None;
        }

        let owner = game.get_player_by_id(self.owner?).unwrap();

        Some(match self.card.card_type {
            CardType::ColoredCard {
                house_rent, color, ..
            } => {
                let houses = self.houses.unwrap();
                if houses == 0 {
                    let group_cards_owned = owner
                        .property
                        .iter()
                        .filter(|p| p.card.color() == Some(color))
                        .count();

                    if group_cards_owned == color.max_cards() {
                        house_rent[0] * 2
                    } else {
                        house_rent[0]
                    }
                } else {
                    house_rent[houses]
                }
            }
            CardType::UtilCard { .. } => self.card.rent,
        })
    }
}
