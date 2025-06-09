#[cfg(feature = "display")]
use cards::{Card, Rank, Suit};

fn main() {
    // Only run this example with the display feature
    #[cfg(not(feature = "display"))]
    {
        println!("This example requires the 'display' feature.");
        println!("Run with: cargo run --example all_cards_large --features display");
        return;
    }

    #[cfg(feature = "display")]
    {
        use cards::display::LargeCardsDisplay;

        println!("All Cards in Large Display Format (5 per line)\n");

        // Create all cards in order
        let mut all_cards = Vec::new();

        // All suits
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        // All ranks (excluding Joker which is handled separately)
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        // Create all cards
        for &suit in &suits {
            for &rank in &ranks {
                all_cards.push(Card::new(rank, Some(suit)));
            }
        }

        // Add jokers if the feature is enabled
        #[cfg(feature = "jokers")]
        {
            all_cards.push(Card::new(Rank::Joker, None));
            all_cards.push(Card::new(Rank::Joker, None));
        }

        // Display cards 5 per line
        for chunk in all_cards.chunks(5) {
            println!("{}", chunk.to_vec().to_large_cards());
            println!(); // Add an empty line between rows for better readability
        }
    }
}
