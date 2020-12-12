use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl Suit {
    pub fn list() -> Vec<Self> {
        vec![Self::Diamonds, Self::Clubs, Self::Hearts, Self::Spades]
    }
}

#[cfg(test)]
mod tests {
    use super::Suit;

    #[test]
    fn test_equality() {
        assert_eq!(Suit::Diamonds, Suit::Diamonds);
        assert_eq!(Suit::Clubs, Suit::Clubs);
        assert_eq!(Suit::Hearts, Suit::Hearts);
        assert_eq!(Suit::Spades, Suit::Spades);
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Suit::list(),
            vec![Suit::Diamonds, Suit::Clubs, Suit::Hearts, Suit::Spades]
        )
    }
}
