use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, Component)]
struct Card {
    suit: Suit,
    value: u8,
    chopping_block: bool
}

#[derive(Debug, Resource)]
struct GameState {
    deck: Vec<Card>,
    hand: Vec<Card>,
    on_deck: Card,
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
            Suit::Heart => "hearts",
            Suit::Diamond => "diamonds",
            Suit::Spade => "spades",
            Suit::Club => "clubs",
        };
        write!(f, "{}", suit_str)
    }
}

#[derive(Resource)]
struct CountdownTimer(Timer);

#[derive(Resource)]
struct Popup {
    show_popup: bool,
    win: bool,
}

fn init_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    let mut value: u8 = 1;
    let mut suit: u8 = 0;
    while suit < 4 {
        while value < 14 {
        let card = match suit {
                0 => Card { suit: Suit::Heart, value, chopping_block: false },
                1 => Card { suit: Suit::Diamond, value, chopping_block: false },
                2 => Card { suit: Suit::Spade, value, chopping_block: false },
                3 => Card { suit: Suit::Club, value, chopping_block: false },
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

fn setup(mut commands: Commands, mut context: EguiContexts) {
    egui_extras::install_image_loaders(context.ctx_mut());
    commands.insert_resource(GameState {
        deck: init_deck(),
        hand: Vec::new(),
        on_deck: Card { value: 1, suit: Suit::Heart, chopping_block: false },
    });
    commands.insert_resource(Popup { show_popup: false, win: false } );
    commands.insert_resource(CountdownTimer(Timer::from_seconds(0.75, TimerMode::Once)))

}

fn check_cards(mut game_state: ResMut<GameState>, mut timer: ResMut<CountdownTimer>, time: Res<Time>, mut popup: ResMut<Popup>) {
    let length = game_state.hand.len();
    if length > 3 {
        let first = game_state.hand[length -1];
        let fourth = game_state.hand[length -4];

        if first.value == fourth.value {
            //Turn cards light red
            game_state.hand[length-1].chopping_block = true;
            game_state.hand[length-2].chopping_block = true;
            game_state.hand[length-3].chopping_block = true;
            game_state.hand[length-4].chopping_block = true;
            //wait .5 seconds
            timer.0.tick(time.delta());
            if timer.0.finished() {
                for _ in 0..4 {
                    game_state.hand.pop();
                }
                timer.0.reset();
            }
        } else if first.suit == fourth.suit {
            //Turn cards light red
            game_state.hand[length-2].chopping_block = true;
            game_state.hand[length-3].chopping_block = true;
            //wait .5 seconds
            timer.0.tick(time.delta());
            if timer.0.finished() {
                game_state.hand.remove(length - 2);
                game_state.hand.remove(length - 3);
                timer.0.reset();
            }
        } else {
            if game_state.deck.is_empty(){
                popup.show_popup = true;
                if game_state.hand.is_empty() { 
                    popup.win = true;
                    game_state.hand.clear();
                    game_state.deck = init_deck();
                } else {
                    popup.win = false;
                    game_state.hand.clear();
                    game_state.deck = init_deck();
                }
            }
        }
    }
}


fn my_ui(mut contexts: EguiContexts, mut game_state: ResMut<GameState>, mut popup: ResMut<Popup>, timer: ResMut<CountdownTimer>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    //egui::Image::new(egui::include_image!("../assets/background.png")).paint_at(ui, ui.ctx().screen_rect());
    ui.heading("Once in a Lifetime");
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(format!("Cards left: {}", game_state.deck.len()));
            if timer.0.elapsed_secs() == 0.0 {
                if ui.button("Draw").clicked() {
                    if let Some(card) = game_state.deck.pop() {
                        game_state.hand.push(card);
                    }
                }
            }
        });

        ui.vertical(|ui| {
            // Display on Deck
            ui.separator();
            ui.label("Card on Deck");
            if game_state.deck.len() > 0 {
                game_state.on_deck = game_state.deck[game_state.deck.len()-1].clone();
                match game_state.on_deck.suit {
                    Suit::Heart => {
                        match game_state.on_deck.value {
                            1 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.125)),
                            2 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.125)),
                            3 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.125)),
                            4 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.125)),
                            5 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.125)),
                            6 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.125)),
                            7 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.125)),
                            8 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.125)),
                            9 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.125)),
                            10 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.125)),
                            11 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.125)),
                            12 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.125)),
                            13 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.125)),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Diamond => {
                        match game_state.on_deck.value {
                            1 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.125)),
                            2 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.125)),
                            3 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.125)),
                            4 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.125)),
                            5 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.125)),
                            6 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.125)),
                            7 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.125)),
                            8 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.125)),
                            9 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.125)),
                            10 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.125)),
                            11 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.125)),
                            12 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.125)),
                            13 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.125)),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Spade => {
                        match game_state.on_deck.value {
                            1 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.125)),
                            2 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.125)),
                            3 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.125)),
                            4 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.125)),
                            5 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.125)),
                            6 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.125)),
                            7 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.125)),
                            8 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.125)),
                            9 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.125)),
                            10 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.125)),
                            11 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.125)),
                            12 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.125)),
                            13 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.125)),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Club => {
                        match game_state.on_deck.value {
                            1 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.125)),
                            2 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.125)),
                            3 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.125)),
                            4 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.125)),
                            5 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.125)),
                            6 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.125)),
                            7 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.125)),
                            8 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.125)),
                            9 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.125)),
                            10 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.125)),
                            11 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.125)),
                            12 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.125)),
                            13 => ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.125)),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                };
            } else {
                ui.add(egui::Image::new(egui::include_image!("../assets/cards/empty.png")).fit_to_original_size(0.125));
            }
        });
    });

        ui.separator();
        
        // Display Hand
        ui.label("Hand:");
        ui.horizontal_wrapped(|ui| {
            for card in &game_state.hand {
                match card.suit {
                    Suit::Heart => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.25));
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Diamond => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.25));
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Spade => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.25));
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Club => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED));
                                } else {
                                    ui.add(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.25));
                                }
                            }
                            _ => (),
                        };
                    },
                }
            }
        });
    });
    if popup.show_popup && popup.win {
        egui::Window::new("Congratulations!")
            .collapsible(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("You Win!");
                if ui.button("Play Again").clicked() {
                    popup.show_popup = false;
                }
            });
    } 
    else if popup.show_popup && !popup.win {
        egui::Window::new("Oh No!")
            .collapsible(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label("You Lose!");
                if ui.button("Play Again").clicked() {
                    popup.show_popup = false;
                }
            });
    }
}


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, my_ui)
        .add_systems(Update, check_cards)
        .run();

}
