use super::super::*;

#[test]
#[cfg(not(feature = "jokers"))]
fn test_deck_has_52_cards_without_jokers() {
    let deck = Deck::new();
    assert_eq!(deck.len(), 52);
}

#[test]
#[cfg(feature = "jokers")]
fn test_deck_has_54_cards_with_jokers() {
    let deck = Deck::new();
    assert_eq!(deck.len(), 54);
}

#[test]
fn test_deck_draw_reduces_length() {
    let mut deck = Deck::new();
    let original_len = deck.len();
    deck.draw();
    assert_eq!(deck.len(), original_len - 1);
}

#[test]
fn test_reset_restores_full_deck() {
    let mut deck = Deck::new();
    deck.draw();
    deck.draw();
    deck.reset();
    #[cfg(feature = "jokers")]
    assert_eq!(deck.len(), 54);
    #[cfg(not(feature = "jokers"))]
    assert_eq!(deck.len(), 52);
}

#[test]
fn test_card_creation() {
    let card = Card::new(Rank::Ace, Some(Suit::Spades));
    assert_eq!(card.rank, Rank::Ace);
    assert_eq!(card.suit, Some(Suit::Spades));
}

#[test]
#[cfg(feature = "jokers")]
fn test_joker_card() {
    let joker = Card::new(Rank::Joker, None);
    assert!(joker.is_joker());
}
