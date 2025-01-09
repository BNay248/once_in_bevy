use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Copy, Clone, Debug)]
struct Card {
    suit: Suit,
    value: u8
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit{
    Suitless,
    Heart,
    Diamond,
    Spade,
    Club,
}

fn play (mut deck: [Card; 52]) {
    // Shuffle Deck
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    // Create hand
    let mut hand = Vec::new();
    // Add first 3 cards
    for num in 0..3 {
        hand.push(deck[num]);
    }

    // Start game
    for el in deck.iter().skip(3) {
        hand.push(*el);
        
        // Draw again if hand too small
        if hand.len() < 4 { continue; }

        // Check value of top and 4th card
        if hand[hand.len()-1].value == hand[hand.len()-4].value {
            // If match, remove top 4 cards from hand
            for _ in 0..4 {
                hand.pop();
            }
            continue;
        }

        // Check suit of top and 4th card
        if hand[hand.len()-1].suit == hand[hand.len()-4].suit {
            //If match, remove middle 2 cards from hand
            hand.remove(hand.len()-2);
            hand.remove(hand.len()-2);
        }
    }

    match hand.len() {
        0 => println!("You win!"),
        _ => println!("You lose!"),
    }
}

fn main() {

    let mut deck = [Card { suit: Suit::Suitless, value: 0 }; 52];

    // Init deck
    let mut deck_position = 0;
    let mut value: u8 = 1;
    let mut suit = 0;
    while suit < 4{
        while value < 14 {
            match suit {

                0 => {
                    deck[deck_position] = Card {
                        suit: Suit::Heart,
                        value: value,
                    };
                    value += 1;
                    deck_position += 1;
                }

                1 => {
                    deck[deck_position] = Card {
                        suit: Suit::Diamond,
                        value: value,
                    };
                    value += 1;
                    deck_position += 1;
                }

                2 => {
                    deck[deck_position] = Card {
                        suit: Suit::Spade,
                        value: value,
                    };
                    value += 1;
                    deck_position += 1;
                }

                3 => {
                    deck[deck_position] = Card {
                        suit: Suit::Club,
                        value: value,
                    };
                    value += 1;
                    deck_position += 1;
                }

                _ => (),
            }
        }
        value = 1;
        suit += 1;
    }
    
    play(deck.clone());

}
