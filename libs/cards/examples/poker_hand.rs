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
        use cards::display::UnicodeDisplay;
        
        println!("\nYour poker hand:");
        println!("┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐");
        
        // Print card values
        print!("│");
        for card in &hand {
            #[cfg(feature = "jokers")]
            if card.rank == cards::Rank::Joker {
                print!(" 🃏 │");
                continue;
            }
            
            let rank = card.rank.to_unicode();
            if rank.len() == 1 {
                print!(" {} │", rank);
            } else {
                print!("{} │", rank);
            }
        }
        println!("");
        
        // Print suits
        print!("│");
        for card in &hand {
            #[cfg(feature = "jokers")]
            if card.rank == cards::Rank::Joker {
                print!("   │");
                continue;
            }
            
            if let Some(suit) = card.suit {
                print!(" {} │", suit.to_unicode());
            } else {
                print!("   │");
            }
        }
        println!("");
        
        println!("└─────┘└─────┘└─────┘└─────┘└─────┘");
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