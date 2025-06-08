use cards::{Deck, Card};

fn main() {
    // Create a new shuffled deck
    let mut deck = Deck::new_shuffled();
    
    println!("Shuffled a new deck with {} cards", deck.len());
    
    // Deal a poker hand of 5 cards
    let mut hand: Vec<Card> = Vec::with_capacity(5);
    
    println!("\nDealing a poker hand...");
    for _ in 0..5 {
        if let Some(card) = deck.draw() {
            hand.push(card);
        } else {
            println!("Deck is empty!");
            break;
        }
    }
    
    // Display the hand
    #[cfg(feature = "display")]
    {
        use cards::display::SmallCardsDisplay;
        
        println!("\nYour poker hand:");
        println!("{}", hand.to_small_cards());
    }
    
    #[cfg(not(feature = "display"))]
    {
        println!("\nYour poker hand:");
        for card in &hand {
            println!("Rank: {:?}, Suit: {:?}", card.rank, card.suit);
        }
    }
    
    println!("\nRemaining cards in deck: {}", deck.len());
}