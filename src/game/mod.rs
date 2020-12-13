pub mod board;
pub mod hands;
mod impl_game;

pub use crate::game::board::Board;
pub use crate::game::hands::{Hand, HandOf2, HandOf4, HandOf5};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Game {
    TexasHoldem(Board, Vec<HandOf2>),
    OmahaHoldem(Board, Vec<HandOf4>),
    FiveCardDraw(Vec<HandOf5>),
}
