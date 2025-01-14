mod images;
use std::sync::Arc;
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
struct ImageResources {
    on_deck: Vec<Arc<egui::Image<'static>>>,
    display: Vec<Arc<egui::Image<'static>>>,
    chopping_block: Vec<Arc<egui::Image<'static>>>,
}


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
    commands.insert_resource(CountdownTimer(Timer::from_seconds(0.75, TimerMode::Once)));
    commands.insert_resource(ImageResources {
        on_deck: images::load_on_deck(),
        display: images::load_display(),
        chopping_block: images::load_chopping_block(),
    });
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


fn my_ui(mut contexts: EguiContexts, mut game_state: ResMut<GameState>, mut popup: ResMut<Popup>, timer: ResMut<CountdownTimer>, images: Res<ImageResources>) {
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
                            1 => ui.add((*images.on_deck[0]).clone()),
                            2 => ui.add((*images.on_deck[1]).clone()),
                            3 => ui.add((*images.on_deck[2]).clone()),
                            4 => ui.add((*images.on_deck[3]).clone()),
                            5 => ui.add((*images.on_deck[4]).clone()),
                            6 => ui.add((*images.on_deck[5]).clone()),
                            7 => ui.add((*images.on_deck[6]).clone()),
                            8 => ui.add((*images.on_deck[7]).clone()),
                            9 => ui.add((*images.on_deck[8]).clone()),
                            10 => ui.add((*images.on_deck[9]).clone()),
                            11 => ui.add((*images.on_deck[10]).clone()),
                            12 => ui.add((*images.on_deck[11]).clone()),
                            13 => ui.add((*images.on_deck[12]).clone()),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Diamond => {
                        match game_state.on_deck.value {
                            1 => ui.add((*images.on_deck[13]).clone()),
                            2 => ui.add((*images.on_deck[14]).clone()),
                            3 => ui.add((*images.on_deck[15]).clone()),
                            4 => ui.add((*images.on_deck[16]).clone()),
                            5 => ui.add((*images.on_deck[17]).clone()),
                            6 => ui.add((*images.on_deck[18]).clone()),
                            7 => ui.add((*images.on_deck[19]).clone()),
                            8 => ui.add((*images.on_deck[20]).clone()),
                            9 => ui.add((*images.on_deck[21]).clone()),
                            10 => ui.add((*images.on_deck[22]).clone()),
                            11 => ui.add((*images.on_deck[23]).clone()),
                            12 => ui.add((*images.on_deck[24]).clone()),
                            13 => ui.add((*images.on_deck[25]).clone()),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Spade => {
                        match game_state.on_deck.value {
                            1 => ui.add((*images.on_deck[26]).clone()),
                            2 => ui.add((*images.on_deck[27]).clone()),
                            3 => ui.add((*images.on_deck[28]).clone()),
                            4 => ui.add((*images.on_deck[29]).clone()),
                            5 => ui.add((*images.on_deck[30]).clone()),
                            6 => ui.add((*images.on_deck[31]).clone()),
                            7 => ui.add((*images.on_deck[32]).clone()),
                            8 => ui.add((*images.on_deck[33]).clone()),
                            9 => ui.add((*images.on_deck[34]).clone()),
                            10 => ui.add((*images.on_deck[35]).clone()),
                            11 => ui.add((*images.on_deck[36]).clone()),
                            12 => ui.add((*images.on_deck[37]).clone()),
                            13 => ui.add((*images.on_deck[38]).clone()),
                            _ => ui.add(egui::Image::new(egui::include_image!("../assets/cards/error.png")).fit_to_original_size(0.125)),
                        };
                    },
                    Suit::Club => {
                        match game_state.on_deck.value {
                            1 => ui.add((*images.on_deck[39]).clone()),
                            2 => ui.add((*images.on_deck[40]).clone()),
                            3 => ui.add((*images.on_deck[41]).clone()),
                            4 => ui.add((*images.on_deck[42]).clone()),
                            5 => ui.add((*images.on_deck[43]).clone()),
                            6 => ui.add((*images.on_deck[44]).clone()),
                            7 => ui.add((*images.on_deck[45]).clone()),
                            8 => ui.add((*images.on_deck[46]).clone()),
                            9 => ui.add((*images.on_deck[47]).clone()),
                            10 => ui.add((*images.on_deck[48]).clone()),
                            11 => ui.add((*images.on_deck[49]).clone()),
                            12 => ui.add((*images.on_deck[50]).clone()),
                            13 => ui.add((*images.on_deck[51]).clone()),
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
                                    ui.add((*images.chopping_block[0]).clone());
                                } else {
                                    ui.add((*images.display[0]).clone());
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[1]).clone());
                                } else {
                                    ui.add((*images.display[1]).clone());
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[2]).clone());
                                } else {
                                    ui.add((*images.display[2]).clone());
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[3]).clone());
                                } else {
                                    ui.add((*images.display[3]).clone());
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[4]).clone());
                                } else {
                                    ui.add((*images.display[4]).clone());
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[5]).clone());
                                } else {
                                    ui.add((*images.display[5]).clone());
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[6]).clone());
                                } else {
                                    ui.add((*images.display[6]).clone());
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[7]).clone());
                                } else {
                                    ui.add((*images.display[7]).clone());
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[8]).clone());
                                } else {
                                    ui.add((*images.display[8]).clone());
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[9]).clone());
                                } else {
                                    ui.add((*images.display[9]).clone());
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[10]).clone());
                                } else {
                                    ui.add((*images.display[10]).clone());
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[11]).clone());
                                } else {
                                    ui.add((*images.display[11]).clone());
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[12]).clone());
                                } else {
                                    ui.add((*images.display[12]).clone());
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Diamond => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[13]).clone());
                                } else {
                                    ui.add((*images.display[13]).clone());
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[14]).clone());
                                } else {
                                    ui.add((*images.display[14]).clone());
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[15]).clone());
                                } else {
                                    ui.add((*images.display[15]).clone());
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[16]).clone());
                                } else {
                                    ui.add((*images.display[16]).clone());
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[17]).clone());
                                } else {
                                    ui.add((*images.display[17]).clone());
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[18]).clone());
                                } else {
                                    ui.add((*images.display[18]).clone());
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[19]).clone());
                                } else {
                                    ui.add((*images.display[19]).clone());
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[20]).clone());
                                } else {
                                    ui.add((*images.display[20]).clone());
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[21]).clone());
                                } else {
                                    ui.add((*images.display[21]).clone());
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[22]).clone());
                                } else {
                                    ui.add((*images.display[22]).clone());
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[23]).clone());
                                } else {
                                    ui.add((*images.display[23]).clone());
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[24]).clone());
                                } else {
                                    ui.add((*images.display[24]).clone());
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[25]).clone());
                                } else {
                                    ui.add((*images.display[25]).clone());
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Spade => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[26]).clone());
                                } else {
                                    ui.add((*images.display[26]).clone());
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[27]).clone());
                                } else {
                                    ui.add((*images.display[27]).clone());
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[28]).clone());
                                } else {
                                    ui.add((*images.display[28]).clone());
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[29]).clone());
                                } else {
                                    ui.add((*images.display[29]).clone());
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[30]).clone());
                                } else {
                                    ui.add((*images.display[30]).clone());
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[31]).clone());
                                } else {
                                    ui.add((*images.display[31]).clone());
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[32]).clone());
                                } else {
                                    ui.add((*images.display[32]).clone());
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[33]).clone());
                                } else {
                                    ui.add((*images.display[33]).clone());
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[34]).clone());
                                } else {
                                    ui.add((*images.display[34]).clone());
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[35]).clone());
                                } else {
                                    ui.add((*images.display[35]).clone());
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[36]).clone());
                                } else {
                                    ui.add((*images.display[36]).clone());
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[37]).clone());
                                } else {
                                    ui.add((*images.display[37]).clone());
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[38]).clone());
                                } else {
                                    ui.add((*images.display[38]).clone());
                                }
                            }
                            _ => (),
                        };
                    },
                    Suit::Club => {
                        match card.value {
                            1 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[39]).clone());
                                } else {
                                    ui.add((*images.display[39]).clone());
                                }
                            }
                            2 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[40]).clone());
                                } else {
                                    ui.add((*images.display[40]).clone());
                                }
                            }
                            3 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[41]).clone());
                                } else {
                                    ui.add((*images.display[41]).clone());
                                }
                            }
                            4 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[42]).clone());
                                } else {
                                    ui.add((*images.display[42]).clone());
                                }
                            }
                            5 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[43]).clone());
                                } else {
                                    ui.add((*images.display[43]).clone());
                                }
                            }
                            6 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[44]).clone());
                                } else {
                                    ui.add((*images.display[44]).clone());
                                }
                            }
                            7 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[45]).clone());
                                } else {
                                    ui.add((*images.display[45]).clone());
                                }
                            }
                            8 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[46]).clone());
                                } else {
                                    ui.add((*images.display[46]).clone());
                                }
                            }
                            9 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[47]).clone());
                                } else {
                                    ui.add((*images.display[47]).clone());
                                }
                            }
                            10 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[48]).clone());
                                } else {
                                    ui.add((*images.display[48]).clone());
                                }
                            }
                            11 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[49]).clone());
                                } else {
                                    ui.add((*images.display[49]).clone());
                                }
                            }
                            12 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[50]).clone());
                                } else {
                                    ui.add((*images.display[50]).clone());
                                }
                            }
                            13 => {
                                if card.chopping_block {
                                    ui.add((*images.chopping_block[51]).clone());
                                } else {
                                    ui.add((*images.display[51]).clone());
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
