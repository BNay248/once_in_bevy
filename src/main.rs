use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug)]
struct Card {
    suit: Suit,
    value: u8
}

#[derive(Default, Debug, Resource)]
struct GameState {
    deck: Vec<Card>,
    hand: Vec<Card>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit{
    Heart,
    Diamond,
    Spade,
    Club,
}

use std::fmt;
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_str = match self {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Spade => "Spade",
            Suit::Club => "Club",
        };
        write!(f, "{}", suit_str)
    }
}

fn init_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    let mut value: u8 = 1;
    let mut suit: u8 = 0;
    while suit < 4 {
        while value < 14 {
        let card = match suit {
                0 => Card { suit: Suit::Heart, value },
                1 => Card { suit: Suit::Diamond, value },
                2 => Card { suit: Suit::Spade, value },
                3 => Card { suit: Suit::Club, value },
                _ => continue,
            };
            deck.push(card);
            value += 1;
        }
        value =1;
        suit += 1;
    }

    // Shuffle
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    deck
}

fn setup(mut commands: Commands) {
    
    commands.insert_resource(GameState {
        deck: init_deck(),
        hand: Vec::new(),
    });
}

fn check_cards(hand: &mut Vec<Card>) {
    let first = hand[hand.len() -1];
    let fourth = hand[hand.len() -4];

    if first.value == fourth.value {
        for _ in 0..4 {
            hand.pop();
        }
    }

    if first.suit == fourth.suit {
        hand.remove(hand.len() - 2);
        hand.remove(hand.len() - 2);
        if hand.len() > 3 {
            check_cards(hand);
        }
    }
}


fn my_ui(mut contexts: EguiContexts, mut game_state: ResMut<GameState>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("Once in a Lifetime");
        
        
        ui.label(format!("Cards left: {}", game_state.deck.len()));
        ui.horizontal(|ui| {
            if ui.button("🂠").clicked() {
                if let Some(card) = game_state.deck.pop() {
                    game_state.hand.push(card);
                    if game_state.hand.len() > 3 {
                        check_cards(&mut game_state.hand);
                    }
                    println!("{:?} Drawn", card);
                } else {
                    println!("No more cards in the deck!");
                    if game_state.deck.is_empty() { 
                        println!("You lose!");
                        game_state.hand.clear();
                        game_state.deck = init_deck();
                    } else {
                        println!("You win!");
                        game_state.hand.clear();
                        game_state.deck = init_deck();
                    }
                }
            }
        });

        ui.separator();

        ui.label("Hand:");

        ui.horizontal(|ui| {
            for card in &game_state.hand {
                ui.add(egui::Button::new(format!("{} of {}s", card.value, card.suit)));
            }
        });
    });
}


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, my_ui)
        .run();

}
