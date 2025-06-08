use crate::{Card, Rank, Suit};
use crate::display::LargeCardsDisplay;

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