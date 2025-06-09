use super::super::*;
use crate::{Card, Rank, Suit};
use proptest::prelude::*;
use proptest::strategy::{BoxedStrategy, Strategy};

fn arb_suit() -> impl Strategy<Value = Suit> {
    prop_oneof![
        Just(Suit::Clubs),
        Just(Suit::Diamonds),
        Just(Suit::Hearts),
        Just(Suit::Spades),
    ]
}

fn arb_rank() -> BoxedStrategy<Rank> {
    let standard_ranks = prop_oneof![
        Just(Rank::Ace),
        Just(Rank::Two),
        Just(Rank::Three),
        Just(Rank::Four),
        Just(Rank::Five),
        Just(Rank::Six),
        Just(Rank::Seven),
        Just(Rank::Eight),
        Just(Rank::Nine),
        Just(Rank::Ten),
        Just(Rank::Jack),
        Just(Rank::Queen),
        Just(Rank::King),
    ];

    #[cfg(not(feature = "jokers"))]
    {
        standard_ranks.boxed()
    }

    #[cfg(feature = "jokers")]
    {
        prop_oneof![standard_ranks, Just(Rank::Joker)].boxed()
    }
}

fn arb_card() -> BoxedStrategy<Card> {
    #[cfg(not(feature = "jokers"))]
    {
        // Without jokers, all cards have a suit
        arb_rank()
            .prop_flat_map(|rank| arb_suit().prop_map(move |suit| Card::new(rank, Some(suit))))
            .boxed()
    }

    #[cfg(feature = "jokers")]
    {
        // With jokers, we need to handle both jokers (no suit) and regular cards (with suit)
        arb_rank()
            .prop_flat_map(|rank| {
                if rank == Rank::Joker {
                    Just(Card::new(rank, None)).boxed()
                } else {
                    arb_suit()
                        .prop_map(move |suit| Card::new(rank, Some(suit)))
                        .boxed()
                }
            })
            .boxed()
    }
}

proptest! {
    #[test]
    fn unicode_card_is_nonempty(card in arb_card()) {
        let repr = card.to_unicode();
        prop_assert!(!repr.trim().is_empty());
    }
}

#[cfg(feature = "jokers")]
proptest! {
    #[test]
    fn joker_card_produces_joker_output(_dummy in Just(())) {
        let joker = Card::new(Rank::Joker, None);
        prop_assert_eq!(joker.to_unicode(), "🃏");
    }
}
