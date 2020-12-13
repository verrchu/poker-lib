mod impl_game;

use std::convert::TryFrom;
use std::convert::TryInto;

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Game {
    TexasHoldem(Board, Vec<HandOf2>),
    OmahaHoldem(Board, Vec<HandOf4>),
    FiveCardDraw(Vec<HandOf5>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board([Card; 5]);

impl TryFrom<Vec<Card>> for Board {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf2([Card; 2]);

impl TryFrom<Vec<Card>> for HandOf2 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf4([Card; 4]);

impl TryFrom<Vec<Card>> for HandOf4 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf5([Card; 5]);

impl TryFrom<Vec<Card>> for HandOf5 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Variant(pub [Card; 5]);

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::card::{Card, Rank, Suit};
    use crate::game::{Board, HandOf5, HandOf4, HandOf2};

    #[test]
    fn test_board_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
            Card(Rank::King, Suit::Diamonds)
        ];

        assert_eq!(
            Board::try_from(cards.to_vec()).unwrap(),
            Board(cards),
        );
    }

    #[test]
    fn test_hand_of_2_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
        ];

        assert_eq!(
            HandOf2::try_from(cards.to_vec()).unwrap(),
            HandOf2(cards),
        );
    }

    #[test]
    fn test_hand_of_4_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
        ];

        assert_eq!(
            HandOf4::try_from(cards.to_vec()).unwrap(),
            HandOf4(cards),
        );
    }

    #[test]
    fn test_hand_of_5_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
            Card(Rank::King, Suit::Diamonds)
        ];

        assert_eq!(
            HandOf5::try_from(cards.to_vec()).unwrap(),
            HandOf5(cards),
        );
    }
}
