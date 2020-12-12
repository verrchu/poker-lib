use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[cfg(test)]
mod tests {
    use ::claim::*;

    use super::Rank;

    #[test]
    fn test_ordering_two() {
        let lhs = Rank::Two;

        assert_eq!(lhs, Rank::Two);
        assert_lt!(lhs, Rank::Three);
        assert_lt!(lhs, Rank::Four);
        assert_lt!(lhs, Rank::Five);
        assert_lt!(lhs, Rank::Six);
        assert_lt!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_three() {
        let lhs = Rank::Three;

        assert_gt!(lhs, Rank::Two);
        assert_eq!(lhs, Rank::Three);
        assert_lt!(lhs, Rank::Four);
        assert_lt!(lhs, Rank::Five);
        assert_lt!(lhs, Rank::Six);
        assert_lt!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_four() {
        let lhs = Rank::Four;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_eq!(lhs, Rank::Four);
        assert_lt!(lhs, Rank::Five);
        assert_lt!(lhs, Rank::Six);
        assert_lt!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_five() {
        let lhs = Rank::Five;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_eq!(lhs, Rank::Five);
        assert_lt!(lhs, Rank::Six);
        assert_lt!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_six() {
        let lhs = Rank::Six;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_eq!(lhs, Rank::Six);
        assert_lt!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_seven() {
        let lhs = Rank::Seven;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_eq!(lhs, Rank::Seven);
        assert_lt!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_eight() {
        let lhs = Rank::Eight;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_eq!(lhs, Rank::Eight);
        assert_lt!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_nine() {
        let lhs = Rank::Nine;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_eq!(lhs, Rank::Nine);
        assert_lt!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_ten() {
        let lhs = Rank::Ten;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_gt!(lhs, Rank::Nine);
        assert_eq!(lhs, Rank::Ten);
        assert_lt!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_jack() {
        let lhs = Rank::Jack;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_gt!(lhs, Rank::Nine);
        assert_gt!(lhs, Rank::Ten);
        assert_eq!(lhs, Rank::Jack);
        assert_lt!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_queen() {
        let lhs = Rank::Queen;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_gt!(lhs, Rank::Nine);
        assert_gt!(lhs, Rank::Ten);
        assert_gt!(lhs, Rank::Jack);
        assert_eq!(lhs, Rank::Queen);
        assert_lt!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_king() {
        let lhs = Rank::King;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_gt!(lhs, Rank::Nine);
        assert_gt!(lhs, Rank::Ten);
        assert_gt!(lhs, Rank::Jack);
        assert_gt!(lhs, Rank::Queen);
        assert_eq!(lhs, Rank::King);
        assert_lt!(lhs, Rank::Ace);
    }

    #[test]
    fn test_ordering_ace() {
        let lhs = Rank::Ace;

        assert_gt!(lhs, Rank::Two);
        assert_gt!(lhs, Rank::Three);
        assert_gt!(lhs, Rank::Four);
        assert_gt!(lhs, Rank::Five);
        assert_gt!(lhs, Rank::Six);
        assert_gt!(lhs, Rank::Seven);
        assert_gt!(lhs, Rank::Eight);
        assert_gt!(lhs, Rank::Nine);
        assert_gt!(lhs, Rank::Ten);
        assert_gt!(lhs, Rank::Jack);
        assert_gt!(lhs, Rank::Queen);
        assert_gt!(lhs, Rank::King);
        assert_eq!(lhs, Rank::Ace);
    }
}
