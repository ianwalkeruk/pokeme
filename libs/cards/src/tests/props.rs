use super::super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn deck_draws_return_valid_cards(mut d in Just(Deck::new())) {
        let len = d.len();
        for _ in 0..len {
            let card = d.draw();
            prop_assert!(card.is_some());
            let c = card.unwrap();
            #[cfg(feature = "jokers")]
            {
                prop_assert!(matches!(c.suit, Some(_) | None));
                prop_assert!(matches!(c.rank,
                    Rank::Ace | Rank::Two | Rank::Three | Rank::Four | Rank::Five |
                    Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten |
                    Rank::Jack | Rank::Queen | Rank::King | Rank::Joker));
            }
            #[cfg(not(feature = "jokers"))]
            {
                prop_assert!(c.suit.is_some());
                prop_assert!(matches!(c.rank,
                    Rank::Ace | Rank::Two | Rank::Three | Rank::Four | Rank::Five |
                    Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten |
                    Rank::Jack | Rank::Queen | Rank::King));
            }
        }
        prop_assert!(d.is_empty());
    }
}
