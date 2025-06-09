use crate::display::{LargeCardsDisplay, UnicodeDisplay};
use crate::{Card, Deck, Rank, Suit};

#[test]
fn test_large_cards_display_empty() {
    let cards: Vec<Card> = Vec::new();
    assert_eq!(cards.to_large_cards(), "No cards");
}

#[test]
fn test_large_cards_display_single_card() {
    let cards = vec![Card::new(Rank::Ace, Some(Suit::Spades))];
    let display = cards.to_large_cards();

    // Check that the display contains the correct elements
    assert!(display.contains("╭───────────╮"));
    assert!(display.contains("│A          │"));
    assert!(display.contains("│♠          │"));
    assert!(display.contains("│  |  ♠  |  │"));
    assert!(display.contains("│          ♠│"));
    assert!(display.contains("│          A│"));
    assert!(display.contains("╰───────────╯"));
}

#[test]
fn test_large_cards_display_multiple_cards() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Spades)),
        Card::new(Rank::King, Some(Suit::Hearts)),
    ];

    let display = cards.to_large_cards();

    // Check that both cards are in the display
    assert!(display.contains("│A          │"));
    assert!(display.contains("│♠          │"));
    assert!(display.contains("│K          │"));
    assert!(display.contains("│♥          │"));
}

#[test]
fn test_large_cards_display_number_cards() {
    // Test a few number cards to ensure pips are displayed correctly
    let cards = vec![
        Card::new(Rank::Two, Some(Suit::Clubs)),
        Card::new(Rank::Five, Some(Suit::Diamonds)),
    ];

    let display = cards.to_large_cards();

    // Check for the correct number of pips
    assert!(display.contains("│2          │"));
    assert!(display.contains("│♣          │"));
    assert!(display.contains("│5          │"));
    assert!(display.contains("│♦          │"));

    // Two should have 2 pips
    let two_pips = display.matches("♣").count();
    assert!(two_pips >= 4); // 2 for the corners + 2 for the pips

    // Five should have 5 pips
    let five_pips = display.matches("♦").count();
    assert!(five_pips >= 7); // 2 for the corners + 5 for the pips
}

#[test]
fn test_large_cards_display_face_cards() {
    // Test face cards to ensure ASCII art is displayed
    let cards = vec![
        Card::new(Rank::Jack, Some(Suit::Spades)),
        Card::new(Rank::Queen, Some(Suit::Hearts)),
        Card::new(Rank::King, Some(Suit::Diamonds)),
    ];

    let display = cards.to_large_cards();

    // Check for face card ASCII art elements
    assert!(display.contains("│  | J   |  │"));
    assert!(display.contains("│  | Q   |  │"));
    assert!(display.contains("│  | K   |  │"));
}

#[cfg(feature = "jokers")]
#[test]
fn test_large_cards_display_joker() {
    let cards = vec![Card::new(Rank::Joker, None)];
    let display = cards.to_large_cards();

    // Check for joker-specific elements
    assert!(display.contains("JOKER"));
    assert!(display.contains("| o o |"));
    assert!(display.contains("|  >  |"));
}

#[test]
fn test_large_cards_dimensions() {
    let deck = Deck::new();

    // Expected dimensions
    const EXPECTED_HEIGHT: usize = 12;
    const EXPECTED_WIDTH: usize = 13; // Including the border characters

    for card in deck.cards.iter() {
        let display = vec![*card].to_large_cards();
        let lines: Vec<&str> = display.lines().collect();

        // 1. Ensure the card has the right number of lines
        assert_eq!(
            lines.len(),
            EXPECTED_HEIGHT,
            "Card {:?} has incorrect number of lines: expected {}, got {}",
            card,
            EXPECTED_HEIGHT,
            lines.len()
        );

        // 2. Ensure every line is the right width
        for (i, line) in lines.iter().enumerate() {
            // Count the actual characters (Unicode aware)
            let char_count = line.chars().count();

            assert_eq!(
                char_count, EXPECTED_WIDTH,
                "Card {:?} line {} has incorrect width: expected {}, got {}",
                card, i, EXPECTED_WIDTH, char_count
            );
        }
    }
}

#[test]
fn test_large_cards_borders() {
    let deck = Deck::new();

    for card in deck.cards.iter() {
        let display = vec![*card].to_large_cards();
        let lines: Vec<&str> = display.lines().collect();

        // 1. Ensure the top border has the correct corners and lines
        let top_border = lines[0];
        assert_eq!(
            top_border.chars().nth(0).unwrap(),
            '╭',
            "Card {:?} top-left corner is incorrect",
            card
        );
        assert_eq!(
            top_border.chars().last().unwrap(),
            '╮',
            "Card {:?} top-right corner is incorrect",
            card
        );
        assert!(
            top_border.chars().skip(1).take(11).all(|c| c == '─'),
            "Card {:?} top border line is incorrect",
            card
        );

        // 2. Ensure the bottom border has the correct corners and lines
        let bottom_border = lines[lines.len() - 1];
        assert_eq!(
            bottom_border.chars().nth(0).unwrap(),
            '╰',
            "Card {:?} bottom-left corner is incorrect",
            card
        );
        assert_eq!(
            bottom_border.chars().last().unwrap(),
            '╯',
            "Card {:?} bottom-right corner is incorrect",
            card
        );
        assert!(
            bottom_border.chars().skip(1).take(11).all(|c| c == '─'),
            "Card {:?} bottom border line is incorrect",
            card
        );

        // 3. Ensure the lines between the top and bottom have vertical bars as the first and last characters
        for i in 1..(lines.len() - 1) {
            let line = lines[i];
            assert_eq!(
                line.chars().nth(0).unwrap(),
                '│',
                "Card {:?} line {} does not start with vertical bar",
                card,
                i
            );
            assert_eq!(
                line.chars().last().unwrap(),
                '│',
                "Card {:?} line {} does not end with vertical bar",
                card,
                i
            );
        }
    }
}

#[test]
fn test_large_cards_pips() {
    let deck = Deck::new();

    for card in deck.cards.iter() {
        let display = vec![*card].to_large_cards();

        if let Some(suit) = card.suit {
            let suit_symbol = suit.to_unicode();
            let pip_count = display.matches(&suit_symbol).count();

            match card.rank {
                // For Aces and face cards, there should be 2 suit symbols (corners)
                Rank::Ace => {
                    assert_eq!(
                        pip_count, 3,
                        "Ace of {:?} should have 3 suit symbols (2 corners + 1 center), found {}",
                        suit, pip_count
                    );
                }
                Rank::Jack | Rank::Queen | Rank::King => {
                    assert_eq!(
                        pip_count, 2,
                        "{:?} of {:?} should have 2 suit symbols (corners only), found {}",
                        card.rank, suit, pip_count
                    );
                }
                // For Number cards, there should be 2 (corners) + rank value
                Rank::Two => {
                    assert_eq!(
                        pip_count, 4,
                        "Two of {:?} should have 4 suit symbols (2 corners + 2 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Three => {
                    assert_eq!(
                        pip_count, 5,
                        "Three of {:?} should have 5 suit symbols (2 corners + 3 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Four => {
                    assert_eq!(
                        pip_count, 6,
                        "Four of {:?} should have 6 suit symbols (2 corners + 4 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Five => {
                    assert_eq!(
                        pip_count, 7,
                        "Five of {:?} should have 7 suit symbols (2 corners + 5 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Six => {
                    assert_eq!(
                        pip_count, 8,
                        "Six of {:?} should have 8 suit symbols (2 corners + 6 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Seven => {
                    assert_eq!(
                        pip_count, 9,
                        "Seven of {:?} should have 9 suit symbols (2 corners + 7 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Eight => {
                    assert_eq!(
                        pip_count, 10,
                        "Eight of {:?} should have 10 suit symbols (2 corners + 8 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Nine => {
                    assert_eq!(
                        pip_count, 11,
                        "Nine of {:?} should have 11 suit symbols (2 corners + 9 pips), found {}",
                        suit, pip_count
                    );
                }
                Rank::Ten => {
                    assert_eq!(
                        pip_count, 12,
                        "Ten of {:?} should have 12 suit symbols (2 corners + 10 pips), found {}",
                        suit, pip_count
                    );
                }
                #[cfg(feature = "jokers")]
                Rank::Joker => unreachable!("Jokers should not have a suit"),
            }
        } else {
            // For Jokers there should be no suit symbols
            #[cfg(feature = "jokers")]
            if card.rank == Rank::Joker {
                assert!(
                    !display.contains("♠")
                        && !display.contains("♥")
                        && !display.contains("♦")
                        && !display.contains("♣"),
                    "Joker should not have any suit symbols"
                );

                // Check for JOKER text
                assert!(
                    display.contains("JOKER"),
                    "Joker card should contain 'JOKER' text"
                );
            }
        }
    }
}
