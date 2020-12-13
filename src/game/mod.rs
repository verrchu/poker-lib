pub mod board;
pub mod hands;
mod impl_game;

use crate::card::Card;
pub use crate::game::board::Board;
pub use crate::game::hands::{Hand, HandOf2, HandOf4, HandOf5};

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Game {
    TexasHoldem(Board, Vec<HandOf2>),
    OmahaHoldem(Board, Vec<HandOf4>),
    FiveCardDraw(Vec<HandOf5>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variant(pub(crate) [Card; 5]);

impl Variant {
    pub fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for Variant {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}
