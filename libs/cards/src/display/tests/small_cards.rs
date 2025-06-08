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