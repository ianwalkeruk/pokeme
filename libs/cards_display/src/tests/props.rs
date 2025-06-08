use super::super::*;
use cards::{Card, Rank, Suit};
use proptest::prelude::*;
use proptest::strategy::{Strategy,BoxedStrategy};

fn arb_suit() -> impl Strategy<Value = Suit> {
    prop_oneof![
        Just(Suit::Clubs),
        Just(Suit::Diamonds),
        Just(Suit::Hearts),
        Just(Suit::Spades),
    ]
}

fn arb_rank() -> BoxedStrategy<Rank> {
    let variants: Vec<Just<Rank>> = vec![
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

    #[cfg(feature = "jokers")]
    variants.push(Just(Rank::Joker).boxed());
    prop_oneof![ranks]
}

fn arb_card() -> impl Strategy<Value = Card> {
    arb_rank().prop_flat_map(|rank| {
        match rank {
            #[cfg(feature="jokers")]
            Rank::Joker => Just(Card::new(rank, None)).boxed(),
            _ => arb_suit().prop_map(move |suit| Card::new(rank, Some(suit))).boxed(),
        }
    })
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
    fn joker_card_produces_joker_output() {
        let joker = Card::new(Rank::Joker, None);
        prop_assert_eq!(joker.to_unicode(), "🃏");
    }
}
