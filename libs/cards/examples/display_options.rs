use cards::{Card, Deck, Rank, Suit};

fn main() {
    // Only run this example with the display feature
    #[cfg(not(feature = "display"))]
    {
        println!("This example requires the 'display' feature.");
        println!("Run with: cargo run --example display_options --features display");
        return;
    }

    #[cfg(feature = "display")]
    {
        use cards::display::{UnicodeDisplay, SmallCardsDisplay, LargeCardsDisplay};
        
        println!("Card Display Options Demo\n");
        
        // 1. Display individual cards with UnicodeDisplay
        println!("1. Individual Card Display (UnicodeDisplay):");
        let ace_spades = Card::new(Rank::Ace, Some(Suit::Spades));
        let king_hearts = Card::new(Rank::King, Some(Suit::Hearts));
        let queen_diamonds = Card::new(Rank::Queen, Some(Suit::Diamonds));
        
        println!("   Ace of Spades: {}", ace_spades.to_unicode());
        println!("   King of Hearts: {}", king_hearts.to_unicode());
        println!("   Queen of Diamonds: {}", queen_diamonds.to_unicode());
        
        // 2. Display a hand with SmallCardsDisplay
        println!("\n2. Hand Display (SmallCardsDisplay):");
        let hand = vec![ace_spades, king_hearts, queen_diamonds];
        println!("{}", hand.to_small_cards());
        
        // 3. Display the same hand with LargeCardsDisplay
        println!("\n3. Hand Display (LargeCardsDisplay):");
        println!("{}", hand.to_large_cards());
        
        // 4. Display a poker hand
        println!("\n4. Poker Hand (5 cards):");
        let mut deck = Deck::new_shuffled();
        let mut poker_hand = Vec::with_capacity(5);
        
        for _ in 0..5 {
            if let Some(card) = deck.draw() {
                poker_hand.push(card);
            }
        }
        
        println!("Small display:");
        println!("{}", poker_hand.to_small_cards());
        
        println!("\nLarge display:");
        println!("{}", poker_hand.to_large_cards());
        
        // 5. Display a full deck by suit
        println!("\n5. Full Deck Display (by suit):");
        
        // Create a new deck and organize cards by suit
        let mut full_deck = Deck::new();
        let all_cards: Vec<Card> = (0..full_deck.len()).filter_map(|_| full_deck.draw()).collect();
        
        // Group cards by suit
        let mut clubs = Vec::new();
        let mut diamonds = Vec::new();
        let mut hearts = Vec::new();
        let mut spades = Vec::new();
        
        for card in all_cards {
            if let Some(suit) = card.suit {
                match suit {
                    Suit::Clubs => clubs.push(card),
                    Suit::Diamonds => diamonds.push(card),
                    Suit::Hearts => hearts.push(card),
                    Suit::Spades => spades.push(card),
                }
            }
        }
        
        // Display each suit (using small cards for brevity)
        println!("\nClubs suit:");
        println!("{}", clubs.to_small_cards());
        
        println!("\nDiamonds suit:");
        println!("{}", diamonds.to_small_cards());
        
        println!("\nHearts suit:");
        println!("{}", hearts.to_small_cards());
        
        println!("\nSpades suit:");
        println!("{}", spades.to_small_cards());
        
        // 6. Jokers (if enabled)
        #[cfg(feature = "jokers")]
        {
            println!("\n6. Jokers:");
            let jokers = vec![
                Card::new(Rank::Joker, None),
                Card::new(Rank::Joker, None),
            ];
            
            println!("Small display:");
            println!("{}", jokers.to_small_cards());
            
            println!("\nLarge display:");
            println!("{}", jokers.to_large_cards());
        }
    }
}