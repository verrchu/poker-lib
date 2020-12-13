use std::convert::TryFrom;
use std::convert::TryInto;
use std::hash::Hash;

mod impl_combination;
mod impl_ord;

use crate::card::Card;
use crate::card::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Combination {
    HighCard { rank: Rank },
    Pair { rank: Rank, kicker: Rank },
    TwoPairs { low: Rank, high: Rank, kicker: Rank },
    ThreeOfAKind { rank: Rank, kicker: Rank },
    Straight { rank: Rank },
    Flush { rank: Rank },
    FullHouse { two: Rank, three: Rank },
    FourOfAKind { rank: Rank, kicker: Rank },
    StraightFlush { rank: Rank },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variant(pub(crate) [Card; 5]);

impl Variant {
    pub fn cards(&self) -> Vec<Card> {
        self.0.to_vec()
    }
}

impl TryFrom<Vec<Card>> for Variant {
    type Error = std::io::Error;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().unwrap()))
    }
}
