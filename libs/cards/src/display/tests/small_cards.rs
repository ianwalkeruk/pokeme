use crate::{Card, Rank, Suit};
use crate::display::SmallCardsDisplay;

#[test]
fn test_small_cards_display_empty() {
    let cards: Vec<Card> = Vec::new();
    assert_eq!(cards.to_small_cards(), "No cards");
}

#[test]
fn test_small_cards_display_single_card() {
    let cards = vec![Card::new(Rank::Ace, Some(Suit::Spades))];
    let expected = "┌───┐\n│ A │\n│ ♠ │\n└───┘";
    assert_eq!(cards.to_small_cards(), expected);
}

#[test]
fn test_small_cards_display_multiple_cards() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Spades)),
        Card::new(Rank::King, Some(Suit::Hearts)),
        Card::new(Rank::Queen, Some(Suit::Diamonds)),
    ];
    let expected = "┌───┐┌───┐┌───┐\n│ A ││ K ││ Q │\n│ ♠ ││ ♥ ││ ♦ │\n└───┘└───┘└───┘";
    assert_eq!(cards.to_small_cards(), expected);
}

#[test]
fn test_small_cards_display_ten() {
    let cards = vec![Card::new(Rank::Ten, Some(Suit::Clubs))];
    let expected = "┌───┐\n│10 │\n│ ♣ │\n└───┘";
    assert_eq!(cards.to_small_cards(), expected);
}

#[test]
fn test_small_cards_display_all_ranks() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Spades)),
        Card::new(Rank::Two, Some(Suit::Spades)),
        Card::new(Rank::Three, Some(Suit::Spades)),
        Card::new(Rank::Four, Some(Suit::Spades)),
        Card::new(Rank::Five, Some(Suit::Spades)),
        Card::new(Rank::Six, Some(Suit::Spades)),
        Card::new(Rank::Seven, Some(Suit::Spades)),
        Card::new(Rank::Eight, Some(Suit::Spades)),
        Card::new(Rank::Nine, Some(Suit::Spades)),
        Card::new(Rank::Ten, Some(Suit::Spades)),
        Card::new(Rank::Jack, Some(Suit::Spades)),
        Card::new(Rank::Queen, Some(Suit::Spades)),
        Card::new(Rank::King, Some(Suit::Spades)),
    ];
    
    let display = cards.to_small_cards();
    
    // Check that each rank appears in the display
    assert!(display.contains("│ A │"));
    assert!(display.contains("│ 2 │"));
    assert!(display.contains("│ 3 │"));
    assert!(display.contains("│ 4 │"));
    assert!(display.contains("│ 5 │"));
    assert!(display.contains("│ 6 │"));
    assert!(display.contains("│ 7 │"));
    assert!(display.contains("│ 8 │"));
    assert!(display.contains("│ 9 │"));
    assert!(display.contains("│10 │"));
    assert!(display.contains("│ J │"));
    assert!(display.contains("│ Q │"));
    assert!(display.contains("│ K │"));
}

#[test]
fn test_small_cards_display_all_suits() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Clubs)),
        Card::new(Rank::Ace, Some(Suit::Diamonds)),
        Card::new(Rank::Ace, Some(Suit::Hearts)),
        Card::new(Rank::Ace, Some(Suit::Spades)),
    ];
    
    let display = cards.to_small_cards();
    
    // Check that each suit appears in the display
    assert!(display.contains("│ ♣ │"));
    assert!(display.contains("│ ♦ │"));
    assert!(display.contains("│ ♥ │"));
    assert!(display.contains("│ ♠ │"));
}

#[test]
fn test_small_cards_display_format() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Spades)),
        Card::new(Rank::King, Some(Suit::Hearts)),
    ];
    
    let display = cards.to_small_cards();
    let lines: Vec<&str> = display.lines().collect();
    
    // Check the structure of the display
    assert_eq!(lines.len(), 4); // 4 lines: top border, ranks, suits, bottom border
    assert!(lines[0].starts_with("┌───┐")); // Top border
    assert!(lines[1].starts_with("│")); // Ranks
    assert!(lines[2].starts_with("│")); // Suits
    assert!(lines[3].starts_with("└───┘")); // Bottom border
}

#[cfg(feature = "jokers")]
#[test]
fn test_small_cards_display_with_joker() {
    let cards = vec![
        Card::new(Rank::Ace, Some(Suit::Spades)),
        Card::new(Rank::Joker, None),
        Card::new(Rank::Queen, Some(Suit::Diamonds)),
    ];
    let expected = "┌───┐┌───┐┌───┐\n│ A ││ 🃏 ││ Q │\n│ ♠ ││   ││ ♦ │\n└───┘└───┘└───┘";
    assert_eq!(cards.to_small_cards(), expected);
}

#[cfg(feature = "jokers")]
#[test]
fn test_small_cards_display_only_jokers() {
    let cards = vec![
        Card::new(Rank::Joker, None),
        Card::new(Rank::Joker, None),
    ];
    let expected = "┌───┐┌───┐\n│ 🃏 ││ 🃏 │\n│   ││   │\n└───┘└───┘";
    assert_eq!(cards.to_small_cards(), expected);
}