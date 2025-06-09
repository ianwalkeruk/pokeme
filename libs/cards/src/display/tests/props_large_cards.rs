#[cfg(test)]
mod props_large_cards {
    use crate::display::LargeCardsDisplay;
    use crate::{Card, Rank, Suit};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn large_cards_display_has_proper_borders(rank in 1u8..14) {
            // Convert the u8 to a Rank
            let rank = match rank {
                1 => Rank::Ace,
                2 => Rank::Two,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                9 => Rank::Nine,
                10 => Rank::Ten,
                11 => Rank::Jack,
                12 => Rank::Queen,
                13 => Rank::King,
                _ => unreachable!(),
            };

            // Create a card with the rank and a suit
            let card = Card::new(rank, Some(Suit::Spades));
            let cards = vec![card];

            // Get the display string
            let display = cards.to_large_cards();

            // Check for top border
            prop_assert!(display.contains("╭───────────╮"),
                "Card with rank {:?} is missing top border", rank);

            // Check for bottom border
            prop_assert!(display.contains("╰───────────╯"),
                "Card with rank {:?} is missing bottom border", rank);

            // Check for side borders (should have at least 10 vertical bars)
            let side_borders = display.matches("│").count();
            prop_assert!(side_borders >= 10,
                "Card with rank {:?} has insufficient side borders: {}", rank, side_borders);

            // Check that the display has the correct number of lines
            let lines = display.lines().count();
            prop_assert_eq!(lines, 12,
                "Card with rank {:?} should have exactly 12 lines, but has {}", rank, lines);
        }

        #[cfg(feature = "jokers")]
        #[test]
        fn joker_card_has_proper_borders(_dummy in 0..1) {
            // Create a joker card
            let card = Card::new(Rank::Joker, None);
            let cards = vec![card];

            // Get the display string
            let display = cards.to_large_cards();

            // Check for top border
            prop_assert!(display.contains("╭───────────╮"),
                "Joker card is missing top border");

            // Check for bottom border
            prop_assert!(display.contains("╰───────────╯"),
                "Joker card is missing bottom border");

            // Check for side borders (should have at least 10 vertical bars)
            let side_borders = display.matches("│").count();
            prop_assert!(side_borders >= 10,
                "Joker card has insufficient side borders: {}", side_borders);

            // Check that the display has the correct number of lines
            let lines = display.lines().count();
            prop_assert_eq!(lines, 12,
                "Joker card should have exactly 12 lines, but has {}", lines);
        }

        #[test]
        fn large_cards_display_has_correct_pip_count(rank in 1u8..10) {
            // Convert the u8 to a Rank
            let rank = match rank {
                1 => Rank::Ace,
                2 => Rank::Two,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                9 => Rank::Nine,
                10 => Rank::Ten,
                _ => unreachable!(),
            };

            // Create a card with the rank and a suit
            let card = Card::new(rank, Some(Suit::Spades));
            let cards = vec![card];

            // Get the display string
            let display = cards.to_large_cards();

            // Count the number of suit symbols (♠)
            let suit_count = display.matches("♠").count();

            // For number cards, we should have:
            // - 2 for the corners (top left and bottom right)
            // - n for the pips (where n is the rank number, except for Ace which has 1 in the center)
            let expected_count = match rank {
                Rank::Ace => 3,   // 2 corners + 1 in the center of the ASCII art
                Rank::Two => 4,   // 2 corners + 2 pips
                Rank::Three => 5, // 2 corners + 3 pips
                Rank::Four => 6,  // 2 corners + 4 pips
                Rank::Five => 7,  // 2 corners + 5 pips
                Rank::Six => 8,   // 2 corners + 6 pips
                Rank::Seven => 9, // 2 corners + 7 pips
                Rank::Eight => 10, // 2 corners + 8 pips
                Rank::Nine => 11, // 2 corners + 9 pips
                Rank::Ten => 12,  // 2 corners + 10 pips
                _ => unreachable!(),
            };

            // Check that the count matches the expected count
            prop_assert_eq!(suit_count, expected_count,
                "Card with rank {:?} should have {} suit symbols, but found {}",
                rank, expected_count, suit_count);
        }

        #[test]
        fn large_cards_display_face_cards_have_correct_format(rank in 11u8..14) {
            // Convert the u8 to a Rank
            let rank = match rank {
                11 => Rank::Jack,
                12 => Rank::Queen,
                13 => Rank::King,
                _ => unreachable!(),
            };

            // Create a card with the rank and a suit
            let card = Card::new(rank, Some(Suit::Hearts));
            let cards = vec![card];

            // Get the display string
            let display = cards.to_large_cards();

            // Face cards should have their rank letter in the display
            let rank_letter = match rank {
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                _ => unreachable!(),
            };

            // Check that the rank letter appears in the card
            prop_assert!(display.contains(&format!("| {} ", rank_letter)),
                "Face card with rank {:?} should have '| {} ' in its display",
                rank, rank_letter);

            // Check that we have exactly 2 suit symbols (2 for corners)
            let suit_count = display.matches("♥").count();
            prop_assert_eq!(suit_count, 2,
                "Face card should have exactly 2 suit symbols (for corners), but found {}",
                suit_count);
        }

        #[cfg(feature = "jokers")]
        #[test]
        fn large_cards_display_joker_has_correct_format(_dummy in 0..1) {
            // Create a joker card
            let card = Card::new(Rank::Joker, None);
            let cards = vec![card];

            // Get the display string
            let display = cards.to_large_cards();

            // Check that "JOKER" appears twice in the display
            let joker_count = display.matches("JOKER").count();
            prop_assert_eq!(joker_count, 2,
                "Joker card should have 'JOKER' appear twice, but found it {} times",
                joker_count);

            // Check for the joker face
            prop_assert!(display.contains("| o o |"),
                "Joker card should have '| o o |' in its display");
            prop_assert!(display.contains("|  >  |"),
                "Joker card should have '|  >  |' in its display");
        }
    }
}
