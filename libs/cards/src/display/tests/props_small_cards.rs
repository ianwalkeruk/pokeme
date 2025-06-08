use crate::{Card, Rank, Suit};
use crate::display::SmallCardsDisplay;
use proptest::prelude::*;

// Helper function to generate arbitrary cards
fn arb_card() -> impl Strategy<Value = Card> {
    let arb_rank = prop_oneof![
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
        Just(Rank::King)
    ];
    
    let arb_suit = prop_oneof![
        Just(Some(Suit::Clubs)),
        Just(Some(Suit::Diamonds)),
        Just(Some(Suit::Hearts)),
        Just(Some(Suit::Spades))
    ];
    
    (arb_rank, arb_suit).prop_map(|(rank, suit)| Card::new(rank, suit))
}

#[cfg(feature = "jokers")]
fn arb_card_with_jokers() -> impl Strategy<Value = Card> {
    prop_oneof![
        arb_card(),
        Just(Card::new(Rank::Joker, None))
    ]
}

proptest! {
    // Test that any valid card produces a non-empty string
    #[test]
    fn small_cards_display_is_nonempty(cards in prop::collection::vec(arb_card(), 1..10)) {
        let display = cards.to_small_cards();
        prop_assert!(!display.is_empty());
    }
    
    // Test that the display contains the correct number of card borders
    #[test]
    fn small_cards_display_has_correct_borders(cards in prop::collection::vec(arb_card(), 1..10)) {
        let display = cards.to_small_cards();
        let top_borders = display.matches("┌───┐").count();
        let bottom_borders = display.matches("└───┘").count();
        
        prop_assert_eq!(top_borders, cards.len());
        prop_assert_eq!(bottom_borders, cards.len());
    }
    
    // Test that the display contains the correct number of lines
    #[test]
    fn small_cards_display_has_correct_lines(cards in prop::collection::vec(arb_card(), 1..10)) {
        let display = cards.to_small_cards();
        let lines: Vec<&str> = display.lines().collect();
        
        // Should have 4 lines: top border, ranks, suits, bottom border
        prop_assert_eq!(lines.len(), 4);
    }
}

#[cfg(feature = "jokers")]
proptest! {
    // Test with jokers
    #[test]
    fn small_cards_display_with_jokers(cards in prop::collection::vec(arb_card_with_jokers(), 1..10)) {
        let display = cards.to_small_cards();
        
        // Count jokers in the original cards
        let joker_count = cards.iter()
            .filter(|card| card.rank == Rank::Joker)
            .count();
            
        // Count joker symbols in the display
        let joker_symbols = display.matches("🃏").count();
        
        prop_assert_eq!(joker_count, joker_symbols);
    }
}