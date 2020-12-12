pub mod rank;
pub use rank::Rank;
pub mod suit;
pub use suit::Suit;

use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card(pub(crate) Rank, pub(crate) Suit);
