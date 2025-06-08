use crate::{Card, Rank};
use super::UnicodeDisplay;

/// Extension trait for Vec<Card> to display cards in a large format with pips and ASCII art
///
/// # Examples
///
/// ```
/// use cards::{Card, Rank, Suit};
/// use cards::display::LargeCardsDisplay;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Some(Suit::Spades)),
///     Card::new(Rank::King, Some(Suit::Hearts)),
/// ];
///
/// let display = cards.to_large_cards();
/// println!("{}", display);
/// // Output:
/// // ╭───────────╮╭───────────╮
/// // │A          ││K          │
/// // │♠          ││♥          │
/// // │           ││           │
/// // │    ___    ││   _____   │
/// // │   /   \   ││  |/|\|\|  │
/// // │  |  ♠  |  ││  | K   |  │
/// // │   \___/   ││  |\|\|/|  │
/// // │           ││  |_____|  │
/// // │          ♠││          ♥│
/// // │          A││          K│
/// // ╰───────────╯╰───────────╯
/// ```
pub trait LargeCardsDisplay {
    /// Returns a formatted string with a large representation of the cards
    ///
    /// The representation includes:
    /// - A detailed card layout with borders
    /// - Rank and suit in the top-left and bottom-right corners
    /// - For number cards (2-10): The correct number of suit pips
    /// - For face cards (J, Q, K): ASCII art representations
    /// - For aces: A single large central pip
    /// - For jokers: A special joker face design
    ///
    /// If the vector is empty, returns "No cards".
    fn to_large_cards(&self) -> String;
}

impl LargeCardsDisplay for Vec<Card> {
    fn to_large_cards(&self) -> String {
        if self.is_empty() {
            return String::from("No cards");
        }

        const CARD_HEIGHT: usize = 12;
        
        // Initialize the result with empty strings for each line
        let mut result_lines: Vec<String> = vec![String::new(); CARD_HEIGHT];
        
        // Process each card
        for card in self {
            // Get card representation
            let mut card_lines = get_large_card_representation(card);
            
            // Ensure we have exactly CARD_HEIGHT lines
            while card_lines.len() < CARD_HEIGHT {
                if card_lines.len() == CARD_HEIGHT - 1 {
                    // Add bottom border if it's missing
                    card_lines.push("╰───────────╯".to_string());
                } else {
                    // Add empty line
                    card_lines.push("│           │".to_string());
                }
            }
            
            // Add each line to the result
            for (i, line) in card_lines.iter().enumerate() {
                if i < CARD_HEIGHT {
                    result_lines[i].push_str(line);
                }
            }
        }
        
        // Join all lines with newlines
        result_lines.join("\n")
    }
}

// Helper function to get the representation of a single card
fn get_large_card_representation(card: &Card) -> Vec<String> {
    const CARD_HEIGHT: usize = 12;
    
    let mut lines = Vec::with_capacity(CARD_HEIGHT);
    
    // Top border with rounded corners
    lines.push("╭───────────╮".to_string());
    
    // Get rank and suit symbols
    let rank_symbol = match card.rank {
        Rank::Ace => "A",
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
        #[cfg(feature = "jokers")]
        Rank::Joker => "🃏",
    };
    
    let suit_symbol = if let Some(suit) = card.suit {
        suit.to_unicode()
    } else {
        " ".to_string()
    };
    
    // Handle jokers differently
    #[cfg(feature = "jokers")]
    if card.rank == Rank::Joker {
        // Joker card
        let mut joker_lines = Vec::with_capacity(CARD_HEIGHT);
        joker_lines.push("╭───────────╮".to_string());
        joker_lines.push("│JOKER      │".to_string());
        joker_lines.push("│           │".to_string());
        joker_lines.push("│           │".to_string());
        joker_lines.push("│    ___    │".to_string());
        joker_lines.push("│   /   \\   │".to_string());
        joker_lines.push("│  | o o |  │".to_string());
        joker_lines.push("│  |  >  |  │".to_string());
        joker_lines.push("│   \\_-_/   │".to_string());
        joker_lines.push("│           │".to_string());
        joker_lines.push("│      JOKER│".to_string());
        joker_lines.push("╰───────────╯".to_string());
        
        return joker_lines;
    }
    
    // Top left rank and suit
    if rank_symbol == "10" {
        lines.push(format!("│{}         │", rank_symbol));
    } else {
        lines.push(format!("│{}          │", rank_symbol));
    }
    
    // Add suit below rank in top left
    lines.push(format!("│{}          │", suit_symbol));
    
    // Middle part with pips or face card art
    match card.rank {
        Rank::Ace => {
            // Ace with ASCII art
            lines.push("│           │".to_string());
            lines.push("│    ___    │".to_string());
            lines.push("│   /   \\   │".to_string());
            lines.push(format!("│  |  {}  |  │", suit_symbol));
            lines.push("│   \\___/   │".to_string());
            lines.push("│           │".to_string());
        },
        Rank::Two => {
            // Two pips
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push("│           │".to_string());
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push("│           │".to_string());
        },
        Rank::Three => {
            // Three pips
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
        },
        Rank::Four => {
            // Four pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
        },
        Rank::Five => {
            // Five pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
        },
        Rank::Six => {
            // Six pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
        },
        Rank::Seven => {
            // Seven pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}  {}  {}  │", suit_symbol, suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
        },
        Rank::Eight => {
            // Eight pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
        },
        Rank::Nine => {
            // Nine pips
            lines.push("│           │".to_string());
            lines.push(format!("│    {} {}    │", suit_symbol, suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│     {}     │", suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│    {} {}    │", suit_symbol, suit_symbol));
        },
        Rank::Ten => {
            // Ten pips
            lines.push("│           │".to_string());
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│    {} {}    │", suit_symbol, suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
            lines.push(format!("│    {} {}    │", suit_symbol, suit_symbol));
            lines.push(format!("│  {}     {}  │", suit_symbol, suit_symbol));
        },
        Rank::Jack => {
            // Jack face
            lines.push("│           │".to_string());
            lines.push("│   _____   │".to_string());
            lines.push("│  |     |  │".to_string());
            lines.push("│  | J   |  │".to_string());
            lines.push("│  |     |  │".to_string());
            lines.push("│  |_____|  │".to_string());
        },
        Rank::Queen => {
            // Queen face
            lines.push("│           │".to_string());
            lines.push("│   _____   │".to_string());
            lines.push("│  /     \\  │".to_string());
            lines.push("│  | Q   |  │".to_string());
            lines.push("│  \\_____/  │".to_string());
            lines.push("│    /_\\    │".to_string());
        },
        Rank::King => {
            // King face
            lines.push("│           │".to_string());
            lines.push("│   _____   │".to_string());
            lines.push("│  |/|\\|\\|  │".to_string());
            lines.push("│  | K   |  │".to_string());
            lines.push("│  |\\|\\|/|  │".to_string());
            lines.push("│  |_____|  │".to_string());
        },
        #[cfg(feature = "jokers")]
        Rank::Joker => {
            // This should never be reached due to the earlier check
            unreachable!();
        },
    }
    
    // Bottom right suit (above rank)
    lines.push(format!("│          {}│", suit_symbol));
    
    // Bottom right rank
    if rank_symbol == "10" {
        lines.push(format!("│         {}│", rank_symbol));
    } else {
        lines.push(format!("│          {}│", rank_symbol));
    }
    
    // Bottom border with rounded corners
    lines.push("╰───────────╯".to_string());
    
    lines
}