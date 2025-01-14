use std::sync::Arc;
pub fn load_on_deck() -> Vec<Arc<egui::Image<'static>>> {
    let mut images = Vec::new();
    //Hearts
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.125))); //0
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.125))); //1
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.125))); //2
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.125))); //3
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.125))); //4
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.125))); //5
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.125))); //6
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.125))); //7
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.125))); //8
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.125))); //9
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.125))); //10
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.125))); //11
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.125))); //12
    //Diamonds
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.125))); //13
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.125))); //14
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.125))); //15
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.125))); //16
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.125))); //17
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.125))); //18
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.125))); //19
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.125))); //20
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.125))); //21
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.125))); //22
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.125))); //23
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.125))); //24
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.125))); //25
    //Spades
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.125))); //26
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.125))); //27
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.125))); //28
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.125))); //29
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.125))); //30
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.125))); //31
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.125))); //32
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.125))); //33
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.125))); //34
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.125))); //35
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.125))); //36
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.125))); //37
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.125))); //38
    //Clubs
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.125))); //39
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.125))); //40
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.125))); //41
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.125))); //42
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.125))); //43
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.125))); //44
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.125))); //45
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.125))); //46
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.125))); //47
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.125))); //48
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.125))); //49
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.125))); //50
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.125))); //51
    images
}

pub fn load_display() -> Vec<Arc<egui::Image<'static>>> {
    let mut images = Vec::new();
    //Hearts
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.25)));
    //Diamonds
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.25)));
    //Spades
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.25)));
    //Clubs
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.25)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.25)));
    images
}

pub fn load_chopping_block() -> Vec<Arc<egui::Image<'static>>> {
    let mut images = Vec::new();
    //Hearts
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_hearts.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    //Diamonds
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_diamonds.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    //Spades
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_spades.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    //Clubs
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/1_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/2_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/3_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/4_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/5_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/6_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/7_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/8_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/9_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/10_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/11_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/12_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images.push(Arc::new(egui::Image::new(egui::include_image!("../assets/cards/13_of_clubs.png")).fit_to_original_size(0.25).tint(egui::Color32::LIGHT_RED)));
    images
}
