use super::super::*;
use crate::{Card, Rank, Suit};

#[test]
fn test_suit_unicode() {
    assert_eq!(Suit::Spades.to_unicode(), "♠");
    assert_eq!(Suit::Hearts.to_unicode(), "♥");
    assert_eq!(Suit::Diamonds.to_unicode(), "♦");
    assert_eq!(Suit::Clubs.to_unicode(), "♣");
}

#[test]
fn test_rank_unicode() {
    assert_eq!(Rank::Ace.to_unicode(), "A");
    assert_eq!(Rank::Ten.to_unicode(), "10");
    assert_eq!(Rank::Queen.to_unicode(), "Q");
}

#[test]
fn test_card_unicode() {
    let card = Card::new(Rank::Jack, Some(Suit::Hearts));
    assert_eq!(card.to_unicode(), "J♥");
}

#[test]
#[cfg(feature = "jokers")]
fn test_joker_unicode() {
    let joker = Card::new(Rank::Joker, None);
    assert_eq!(joker.to_unicode(), "🃏");
}