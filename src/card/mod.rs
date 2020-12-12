pub mod rank;
pub use rank::Rank;
pub mod suit;
pub use suit::Suit;

use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card(pub(crate) Rank, pub(crate) Suit);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card(rank, suit)
    }

    pub fn rank(&self) -> Rank {
        self.0
    }

    pub fn suit(&self) -> Suit {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::Rank;
    use super::Suit;

    #[test]
    fn test_new() {
        assert_eq!(
            Card::new(Rank::Two, Suit::Diamonds),
            Card(Rank::Two, Suit::Diamonds)
        );
    }

    #[test]
    fn test_get_rank() {
        assert_eq!(Card::new(Rank::Two, Suit::Diamonds).rank(), Rank::Two);
    }

    #[test]
    fn test_get_suit() {
        assert_eq!(Card::new(Rank::Two, Suit::Diamonds).suit(), Suit::Diamonds);
    }
}
