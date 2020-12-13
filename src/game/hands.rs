use std::convert::TryFrom;
use std::convert::TryInto;

use crate::card::Card;

pub trait Hand {
    fn cards(&self) -> Vec<Card>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf2([Card; 2]);

impl HandOf2 {
    pub fn new(cards: [Card; 2]) -> Self {
        Self(cards)
    }
}

impl Hand for HandOf2 {
    fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for HandOf2 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf4([Card; 4]);

impl HandOf4 {
    pub fn new(cards: [Card; 4]) -> Self {
        Self(cards)
    }
}

impl Hand for HandOf4 {
    fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for HandOf4 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf5([Card; 5]);

impl HandOf5 {
    pub fn new(cards: [Card; 5]) -> Self {
        Self(cards)
    }
}

impl Hand for HandOf5 {
    fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for HandOf5 {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{HandOf2, HandOf4, HandOf5};
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_hand_of_2_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
        ];

        assert_eq!(HandOf2::try_from(cards.to_vec()).unwrap(), HandOf2(cards),);
    }

    #[test]
    fn test_hand_of_4_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
        ];

        assert_eq!(HandOf4::try_from(cards.to_vec()).unwrap(), HandOf4(cards),);
    }

    #[test]
    fn test_hand_of_5_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
            Card(Rank::King, Suit::Diamonds),
        ];

        assert_eq!(HandOf5::try_from(cards.to_vec()).unwrap(), HandOf5(cards),);
    }
}
