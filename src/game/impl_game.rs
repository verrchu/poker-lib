use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

use ::itertools::Itertools;

use crate::card::Card;
use crate::combination::Combination;
use crate::combination::Variant;
use crate::game::Board;
use crate::game::Game;
use crate::game::Hand;
use crate::game::HandOf2;
use crate::game::HandOf4;

impl Game {
    pub fn rank_hands(game: Self) -> Vec<(Vec<Card>, Combination, Variant)> {
        match game {
            Self::TexasHoldem(board, hands) => hands
                .into_iter()
                .map(|hand| (hand.cards(), Self::texas_holdem_combination(board, hand)))
                .map(|(hand, (combination, variant))| (hand, combination, variant))
                .collect::<Vec<_>>(),
            Self::OmahaHoldem(board, hands) => hands
                .into_iter()
                .map(|hand| (hand.cards(), Self::omaha_holdem_combination(board, hand)))
                .map(|(hand, (combination, variant))| (hand, combination, variant))
                .collect::<Vec<_>>(),
            Self::FiveCardDraw(hands) => hands
                .iter()
                .map(|hand| (hand, Variant::try_from(hand.cards()).unwrap()))
                .map(|(hand, variant)| (hand.cards(), Combination::from_variant(variant), variant))
                .collect::<Vec<_>>(),
        }
    }

    pub fn group_hands(
        hands: Vec<(Vec<Card>, Combination)>,
    ) -> HashMap<Combination, Vec<Vec<Card>>> {
        hands
            .into_iter()
            .fold(HashMap::new(), |mut acc, (hand, comb)| {
                let hands = acc.get_mut(&comb).map_or_else(
                    || vec![hand.clone()],
                    |hands| {
                        hands.push(hand.clone());
                        hands.to_vec()
                    },
                );

                acc.insert(comb, hands);

                acc
            })
    }

    pub fn sort_hands(grouped_hands: HashMap<Combination, Vec<Vec<Card>>>) -> Vec<Vec<Vec<Card>>> {
        grouped_hands
            .into_iter()
            .sorted_by(|(comb_a, _), (comb_b, _)| comb_a.cmp(comb_b))
            .map(|(_comb, hands)| hands)
            .collect()
    }

    fn texas_holdem_combination(board: Board, hand: HandOf2) -> (Combination, Variant) {
        board
            .cards()
            .iter()
            .chain(hand.cards().iter())
            .copied()
            .combinations(5)
            .map(|comb| Variant(comb.try_into().unwrap()))
            .map(|variant| (Combination::from_variant(variant), variant))
            .max_by(|(comb_a, _), (comb_b, _)| comb_a.cmp(comb_b))
            .unwrap()
    }

    fn omaha_holdem_combination(board: Board, hand: HandOf4) -> (Combination, Variant) {
        hand.cards()
            .iter()
            .combinations(2)
            .cartesian_product(board.cards().iter().combinations(3))
            .map(|(h, b)| {
                h.into_iter()
                    .chain(b.into_iter())
                    .copied()
                    .collect::<Vec<_>>()
            })
            .map(|cards| Variant(cards.try_into().unwrap()))
            .map(|variant| (Combination::from_variant(variant), variant))
            .max_by(|(comb_a, _), (comb_b, _)| comb_a.cmp(comb_b))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use ::itertools::Itertools;

    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suit;
    use crate::combination::Combination;
    use crate::combination::Variant;
    use crate::game::Board;
    use crate::game::Game;
    use crate::game::HandOf2;
    use crate::game::HandOf4;
    use crate::game::HandOf5;

    #[test]
    fn test_texas_holdem_ordering() {
        let game = Game::TexasHoldem(
            Board::new([
                Card(Rank::Queen, Suit::Spades),
                Card(Rank::King, Suit::Diamonds),
                Card(Rank::King, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
                Card(Rank::Jack, Suit::Diamonds),
            ]),
            vec![
                HandOf2::new([Card(Rank::King, Suit::Hearts), Card(Rank::Two, Suit::Clubs)]),
                HandOf2::new([
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                ]),
                HandOf2::new([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                ]),
                HandOf2::new([
                    Card(Rank::Six, Suit::Diamonds),
                    Card(Rank::Six, Suit::Hearts),
                ]),
            ],
        );

        assert_eq!(
            Game::rank_hands(game)
                .into_iter()
                .sorted_by(
                    |(_hand_a, comb_a, _variant_a), (_hand_b, comb_b, _variant_b)| comb_a
                        .cmp(comb_b)
                )
                .collect::<Vec<_>>(),
            vec![
                (
                    vec![
                        Card(Rank::Six, Suit::Diamonds),
                        Card(Rank::Six, Suit::Hearts),
                    ],
                    Combination::TwoPairs {
                        low: Rank::Six,
                        high: Rank::King,
                        kicker: Rank::Queen
                    },
                    Variant([
                        Card(Rank::Queen, Suit::Spades),
                        Card(Rank::King, Suit::Diamonds),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::Six, Suit::Diamonds),
                        Card(Rank::Six, Suit::Hearts)
                    ])
                ),
                (
                    vec![Card(Rank::King, Suit::Hearts), Card(Rank::Two, Suit::Clubs),],
                    Combination::ThreeOfAKind {
                        rank: Rank::King,
                        kicker: Rank::Queen
                    },
                    Variant([
                        Card(Rank::Queen, Suit::Spades),
                        Card(Rank::King, Suit::Diamonds),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs)
                    ])
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                    ],
                    Combination::Straight { rank: Rank::Ten },
                    Variant([
                        Card(Rank::Queen, Suit::Spades),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::Jack, Suit::Diamonds),
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts)
                    ])
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                    ],
                    Combination::FullHouse {
                        two: Rank::Seven,
                        three: Rank::King
                    },
                    Variant([
                        Card(Rank::King, Suit::Diamonds),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::Seven, Suit::Clubs),
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds)
                    ])
                )
            ]
        );
    }

    #[test]
    fn test_omaha_holdem_ordering() {
        let game = Game::OmahaHoldem(
            Board::new([
                Card(Rank::Queen, Suit::Spades),
                Card(Rank::King, Suit::Diamonds),
                Card(Rank::King, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
                Card(Rank::Jack, Suit::Diamonds),
            ]),
            vec![
                HandOf4::new([
                    Card(Rank::King, Suit::Hearts),
                    Card(Rank::Two, Suit::Clubs),
                    Card(Rank::Eight, Suit::Clubs),
                    Card(Rank::Two, Suit::Diamonds),
                ]),
                HandOf4::new([
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                    Card(Rank::Seven, Suit::Hearts),
                    Card(Rank::Seven, Suit::Spades),
                ]),
                HandOf4::new([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                    Card(Rank::Ace, Suit::Clubs),
                    Card(Rank::Ten, Suit::Clubs),
                ]),
            ],
        );

        assert_eq!(
            Game::rank_hands(game)
                .into_iter()
                .sorted_by(
                    |(_hand_a, comb_a, _variant_a), (_hand_b, comb_b, _variant_b)| comb_a
                        .cmp(comb_b)
                )
                .collect::<Vec<_>>(),
            vec![
                (
                    vec![
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs),
                        Card(Rank::Eight, Suit::Clubs),
                        Card(Rank::Two, Suit::Diamonds),
                    ],
                    Combination::ThreeOfAKind {
                        rank: Rank::King,
                        kicker: Rank::Queen
                    },
                    Variant([
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Diamonds),
                        Card(Rank::Queen, Suit::Spades),
                        Card(Rank::King, Suit::Diamonds),
                        Card(Rank::King, Suit::Spades)
                    ])
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                    ],
                    Combination::Straight { rank: Rank::Ten },
                    Variant([
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                        Card(Rank::Queen, Suit::Spades),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::Jack, Suit::Diamonds)
                    ])
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                        Card(Rank::Seven, Suit::Hearts),
                        Card(Rank::Seven, Suit::Spades),
                    ],
                    Combination::FullHouse {
                        two: Rank::Seven,
                        three: Rank::King
                    },
                    Variant([
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Spades),
                        Card(Rank::King, Suit::Diamonds),
                        Card(Rank::King, Suit::Spades),
                        Card(Rank::Seven, Suit::Clubs)
                    ])
                )
            ]
        );
    }

    #[test]
    fn test_five_card_draw_ordering() {
        let game = Game::FiveCardDraw(vec![
            HandOf5::new([
                Card(Rank::King, Suit::Hearts),
                Card(Rank::Two, Suit::Clubs),
                Card(Rank::Eight, Suit::Clubs),
                Card(Rank::Two, Suit::Diamonds),
                Card(Rank::Two, Suit::Hearts),
            ]),
            HandOf5::new([
                Card(Rank::King, Suit::Clubs),
                Card(Rank::Seven, Suit::Diamonds),
                Card(Rank::Seven, Suit::Hearts),
                Card(Rank::Seven, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
            ]),
            HandOf5::new([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ten, Suit::Hearts),
                Card(Rank::Ace, Suit::Clubs),
                Card(Rank::Ten, Suit::Clubs),
                Card(Rank::Ten, Suit::Diamonds),
            ]),
        ]);

        assert_eq!(
            Game::rank_hands(game)
                .into_iter()
                .sorted_by(
                    |(_hand_a, comb_a, _variant_a), (_hand_b, comb_b, _variant_b)| comb_a
                        .cmp(comb_b)
                )
                .collect::<Vec<_>>(),
            vec![
                (
                    vec![
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs),
                        Card(Rank::Eight, Suit::Clubs),
                        Card(Rank::Two, Suit::Diamonds),
                        Card(Rank::Two, Suit::Hearts),
                    ],
                    Combination::ThreeOfAKind {
                        rank: Rank::Two,
                        kicker: Rank::King
                    },
                    Variant([
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs),
                        Card(Rank::Eight, Suit::Clubs),
                        Card(Rank::Two, Suit::Diamonds),
                        Card(Rank::Two, Suit::Hearts)
                    ])
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                        Card(Rank::Ten, Suit::Diamonds),
                    ],
                    Combination::FullHouse {
                        two: Rank::Ace,
                        three: Rank::Ten
                    },
                    Variant([
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                        Card(Rank::Ten, Suit::Diamonds)
                    ])
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                        Card(Rank::Seven, Suit::Hearts),
                        Card(Rank::Seven, Suit::Spades),
                        Card(Rank::Seven, Suit::Clubs),
                    ],
                    Combination::FourOfAKind {
                        rank: Rank::Seven,
                        kicker: Rank::King
                    },
                    Variant([
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                        Card(Rank::Seven, Suit::Hearts),
                        Card(Rank::Seven, Suit::Spades),
                        Card(Rank::Seven, Suit::Clubs)
                    ])
                )
            ]
        );
    }

    #[test]
    fn test_group_hands() {
        let hands = vec![
            (
                vec![
                    Card(Rank::King, Suit::Hearts),
                    Card(Rank::Eight, Suit::Clubs),
                ],
                Combination::ThreeOfAKind {
                    rank: Rank::Two,
                    kicker: Rank::King,
                },
            ),
            (
                vec![
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                ],
                Combination::FullHouse {
                    two: Rank::Ace,
                    three: Rank::Ten,
                },
            ),
            (
                vec![
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                ],
                Combination::ThreeOfAKind {
                    rank: Rank::Two,
                    kicker: Rank::King,
                },
            ),
        ];

        assert_eq!(
            Game::group_hands(hands)
                .into_iter()
                .sorted_by(|(comb_a, _), (comb_b, _)| comb_a.cmp(comb_b))
                .collect::<Vec<(Combination, Vec<Vec<Card>>)>>(),
            vec![
                (
                    Combination::ThreeOfAKind {
                        rank: Rank::Two,
                        kicker: Rank::King,
                    },
                    vec![
                        vec![
                            Card(Rank::King, Suit::Hearts),
                            Card(Rank::Eight, Suit::Clubs),
                        ],
                        vec![
                            Card(Rank::King, Suit::Clubs),
                            Card(Rank::Seven, Suit::Diamonds),
                        ],
                    ]
                ),
                (
                    Combination::FullHouse {
                        two: Rank::Ace,
                        three: Rank::Ten,
                    },
                    vec![vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                    ]]
                )
            ]
        );
    }

    #[test]
    fn test_sort_hands() {
        let hands = vec![
            (
                vec![
                    Card(Rank::King, Suit::Hearts),
                    Card(Rank::Eight, Suit::Clubs),
                ],
                Combination::ThreeOfAKind {
                    rank: Rank::Two,
                    kicker: Rank::King,
                },
            ),
            (
                vec![
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                ],
                Combination::FullHouse {
                    two: Rank::Ace,
                    three: Rank::Ten,
                },
            ),
            (
                vec![
                    Card(Rank::Ace, Suit::Hearts),
                    Card(Rank::Ten, Suit::Diamonds),
                ],
                Combination::Pair {
                    rank: Rank::Ace,
                    kicker: Rank::Ten,
                },
            ),
            (
                vec![
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                ],
                Combination::ThreeOfAKind {
                    rank: Rank::Two,
                    kicker: Rank::King,
                },
            ),
        ];

        let grouped_hands = Game::group_hands(hands);

        assert_eq!(
            Game::sort_hands(grouped_hands),
            vec![
                vec![vec![
                    Card(Rank::Ace, Suit::Hearts),
                    Card(Rank::Ten, Suit::Diamonds),
                ]],
                vec![
                    vec![
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Eight, Suit::Clubs),
                    ],
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                    ],
                ],
                vec![vec![
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                ]]
            ]
        );
    }

    #[test]
    fn test_texas_holdem_combination() {
        let board = Board::new([
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ten, Suit::Hearts),
            Card(Rank::Nine, Suit::Hearts),
            Card(Rank::Eight, Suit::Hearts),
            Card(Rank::Seven, Suit::Hearts),
        ]);

        let hand = HandOf2::new([
            Card(Rank::Four, Suit::Hearts),
            Card(Rank::Jack, Suit::Diamonds),
        ]);

        assert_eq!(
            Game::texas_holdem_combination(board, hand),
            (
                Combination::Flush { rank: Rank::Ten },
                Variant([
                    Card(Rank::Ten, Suit::Hearts),
                    Card(Rank::Nine, Suit::Hearts),
                    Card(Rank::Eight, Suit::Hearts),
                    Card(Rank::Seven, Suit::Hearts),
                    Card(Rank::Four, Suit::Hearts)
                ])
            )
        );
    }

    #[test]
    fn test_omaha_holdem_combination() {
        let board = Board::new([
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ten, Suit::Hearts),
            Card(Rank::Nine, Suit::Hearts),
            Card(Rank::Eight, Suit::Hearts),
            Card(Rank::Seven, Suit::Hearts),
        ]);

        let hand = HandOf4::new([
            Card(Rank::Four, Suit::Hearts),
            Card(Rank::Jack, Suit::Diamonds),
            Card(Rank::Queen, Suit::Clubs),
            Card(Rank::King, Suit::Diamonds),
        ]);

        assert_eq!(
            Game::omaha_holdem_combination(board, hand),
            (
                Combination::Straight { rank: Rank::Eight },
                Variant([
                    Card(Rank::Jack, Suit::Diamonds),
                    Card(Rank::Queen, Suit::Clubs),
                    Card(Rank::Ten, Suit::Hearts),
                    Card(Rank::Nine, Suit::Hearts),
                    Card(Rank::Eight, Suit::Hearts)
                ])
            )
        );
    }
}
