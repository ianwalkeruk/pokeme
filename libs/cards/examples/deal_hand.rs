use cards::{Deck, Card};

fn main() {
    // Create a new shuffled deck
    let mut deck = Deck::new_shuffled();
    
    println!("Shuffled a new deck with {} cards", deck.len());
    
    // Deal a hand of 5 cards
    let mut hand: Vec<Card> = Vec::with_capacity(5);
    
    println!("\nDealing a hand of 5 cards...");
    for i in 1..=5 {
        if let Some(card) = deck.draw() {
            // If display feature is enabled, show the card with unicode representation
            #[cfg(feature = "display")]
            {
                use cards::display::UnicodeDisplay;
                println!("Card {}: {:?} ({})", i, card.rank, card.to_unicode());
            }
            
            // If display feature is not enabled, just show the card index
            #[cfg(not(feature = "display"))]
            println!("Card {}: Rank {:?}, Suit {:?}", i, card.rank, card.suit);
            
            hand.push(card);
        } else {
            println!("Deck is empty!");
            break;
        }
    }
    
    // Display the complete hand using SmallCardsDisplay
    #[cfg(feature = "display")]
    {
        use cards::display::SmallCardsDisplay;
        println!("\nYour complete hand:");
        println!("{}", hand.to_small_cards());
    }
    
    println!("\nRemaining cards in deck: {}", deck.len());
}