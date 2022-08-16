mod elements;
mod rules;
use crate::elements::Deck;

fn main() {
    let mut deck: Deck = Deck::base_deck();
    for i in 1..10 {
        deck.shuffle(1);
        println!("\nIterazione {}", i + 1);
        let mut k = 0;
        for card in deck.iter() {
            k += 1;
            println!("Card {}: {}", k, card.to_string());
        }
    }
}
