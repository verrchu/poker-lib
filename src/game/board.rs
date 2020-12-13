use std::convert::TryFrom;
use std::convert::TryInto;

use crate::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board([Card; 5]);

impl Board {
    pub fn new(cards: [Card; 5]) -> Self {
        Self(cards)
    }

    pub fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for Board {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Board;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_board_from_vec() {
        let cards = [
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ace, Suit::Clubs),
            Card(Rank::Ace, Suit::Hearts),
            Card(Rank::Ace, Suit::Spades),
            Card(Rank::King, Suit::Diamonds),
        ];

        assert_eq!(Board::try_from(cards.to_vec()).unwrap(), Board(cards));
    }
}
