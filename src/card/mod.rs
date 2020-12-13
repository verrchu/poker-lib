pub mod rank;
pub use rank::Rank;
pub mod suit;
pub use suit::Suit;

use std::cmp::Ordering;
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

#[cfg(test)]
mod tests {
    use ::itertools::Itertools;

    use std::cmp::Ordering;

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

    #[test]
    fn test_ordering() {
        assert_eq!(
            Card(Rank::Two, Suit::Diamonds).cmp(&Card(Rank::Two, Suit::Diamonds)),
            Ordering::Equal
        );
        assert_eq!(
            Card(Rank::Two, Suit::Diamonds).cmp(&Card(Rank::Two, Suit::Hearts)),
            Ordering::Equal
        );
        assert_eq!(
            Card(Rank::Two, Suit::Diamonds).cmp(&Card(Rank::Three, Suit::Hearts)),
            Ordering::Less
        );
        assert_eq!(
            Card(Rank::Three, Suit::Diamonds).cmp(&Card(Rank::Two, Suit::Hearts)),
            Ordering::Greater
        );

        assert_eq!(
            vec![
                Card(Rank::Four, Suit::Diamonds),
                Card(Rank::Two, Suit::Hearts),
                Card(Rank::Four, Suit::Clubs)
            ]
            .into_iter()
            .sorted()
            .collect::<Vec<_>>(),
            vec![
                Card(Rank::Two, Suit::Hearts),
                Card(Rank::Four, Suit::Diamonds),
                Card(Rank::Four, Suit::Clubs)
            ]
        );
    }
}
