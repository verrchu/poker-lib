mod impl_game;

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Game {
    TexasHoldem(Board, Vec<HandOf2>),
    OmahaHoldem(Board, Vec<HandOf4>),
    FiveCardDraw(Vec<HandOf5>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf2([Card; 2]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf4([Card; 4]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf5([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Variant(pub [Card; 5]);
