use cards::{Card, Deck, Rank, Suit};

fn main() {
    // Only run this example with the display feature
    #[cfg(not(feature = "display"))]
    {
        println!("This example requires the 'display' feature.");
        println!("Run with: cargo run --example large_cards --features display");
        return;
    }

    #[cfg(feature = "display")]
    {
        use cards::display::LargeCardsDisplay;
        
        println!("Large Cards Display Demo\n");
        
        // Create a new shuffled deck
        let mut deck = Deck::new_shuffled();
        
        // Deal a poker hand of 5 cards
        let mut hand: Vec<Card> = Vec::with_capacity(5);
        
        println!("Dealing a poker hand...\n");
        for _ in 0..5 {
            if let Some(card) = deck.draw() {
                hand.push(card);
            } else {
                println!("Deck is empty!");
                break;
            }
        }
        
        // Display the hand with large cards
        println!("{}", hand.to_large_cards());
        
        // Show all ranks of one suit
        println!("\nAll ranks of Spades:\n");
        let all_spades = vec![
            Card::new(Rank::Ace, Some(Suit::Spades)),
            Card::new(Rank::Two, Some(Suit::Spades)),
            Card::new(Rank::Three, Some(Suit::Spades)),
        ];
        println!("{}", all_spades.to_large_cards());
        
        println!("\nFace cards:\n");
        let face_cards = vec![
            Card::new(Rank::Jack, Some(Suit::Clubs)),
            Card::new(Rank::Queen, Some(Suit::Hearts)),
            Card::new(Rank::King, Some(Suit::Diamonds)),
        ];
        println!("{}", face_cards.to_large_cards());
        
        // Show jokers if enabled
        #[cfg(feature = "jokers")]
        {
            println!("\nJokers:\n");
            let jokers = vec![
                Card::new(Rank::Joker, None),
                Card::new(Rank::Joker, None),
            ];
            println!("{}", jokers.to_large_cards());
        }
    }
}