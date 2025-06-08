use crate::Card;
#[cfg(feature = "jokers")]
use crate::Rank;
use super::UnicodeDisplay;

/// Extension trait for Vec<Card> to display cards in a small unicode format
///
/// # Examples
///
/// ```
/// use cards::{Card, Rank, Suit};
/// use cards::display::SmallCardsDisplay;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Some(Suit::Spades)),
///     Card::new(Rank::King, Some(Suit::Hearts)),
///     Card::new(Rank::Queen, Some(Suit::Diamonds)),
/// ];
///
/// let display = cards.to_small_cards();
/// println!("{}", display);
/// // Output:
/// // ┌───┐┌───┐┌───┐
/// // │ A ││ K ││ Q │
/// // │ ♠ ││ ♥ ││ ♦ │
/// // └───┘└───┘└───┘
/// ```
pub trait SmallCardsDisplay {
    /// Returns a formatted string with a small unicode representation of the cards
    ///
    /// The representation includes:
    /// - A top border
    /// - The card rank
    /// - The card suit
    /// - A bottom border
    ///
    /// If the vector is empty, returns "No cards".
    fn to_small_cards(&self) -> String;
}

impl SmallCardsDisplay for Vec<Card> {
    fn to_small_cards(&self) -> String {
        if self.is_empty() {
            return String::from("No cards");
        }

        let mut result = String::new();
        
        // Top border
        result.push_str(&format!("┌───┐{}", "┌───┐".repeat(self.len() - 1)));
        result.push('\n');
        
        // Card ranks
        for card in self {
            #[cfg(feature = "jokers")]
            if card.rank == Rank::Joker {
                result.push_str("│ 🃏 │");
                continue;
            }
            
            let rank = card.rank.to_unicode();
            if rank.len() == 1 {
                result.push_str(&format!("│ {rank} │"));
            } else {
                result.push_str(&format!("│{rank} │"));
            }
        }
        result.push('\n');
        
        // Card suits
        for card in self {
            #[cfg(feature = "jokers")]
            if card.rank == Rank::Joker {
                result.push_str("│   │");
                continue;
            }
            
            if let Some(suit) = card.suit {
                result.push_str(&format!("│ {} │", suit.to_unicode()));
            } else {
                result.push_str("│   │");
            }
        }
        result.push('\n');
        
        // Bottom border
        result.push_str(&format!("└───┘{}", "└───┘".repeat(self.len() - 1)));
        
        result
    }
}