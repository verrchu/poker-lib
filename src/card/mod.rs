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
}
