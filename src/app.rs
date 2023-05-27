

use std::collections::HashMap;



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct Word {
    word: String,
    char_count: HashMap<char,i32>,
    score: i32,
    scale: f32,
}

impl Word {
    fn new(word: String) -> Word {
        let mut char_count = HashMap::new();
        for c in word.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        Word { word, char_count, score: 0 , scale: 1.0}
    }
}

impl Default for Word {
    fn default() -> Self {
        Self {
            word: "".to_owned(),
            char_count: HashMap::new(),
            score: 0,
            scale: 1.0,
        }
    }
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    words: Vec<Word>,
    remaining_letters: Vec<char>,
    guessed_letters: Vec<char>,
    current_word: String,
    scale: f32,
}

impl Default for App {
    fn default() -> Self {

    let mut words = include_str!("words.txt")
        .split_whitespace()
        .map(|word| Word::new(word.to_string())).collect::<Vec<Word>>();

    let mut letter_frequency = HashMap::new();
    for w in 0..words.len() {
        for (letter, count) in words[w].char_count.clone() {
            letter_frequency.insert(letter, letter_frequency.get(&letter).unwrap_or(&0) + count);
        }
    }

    for w in 0..words.len() {
        let mut score = 0;
        words[w].score = words[w].word.chars().map(|c| letter_frequency.get(&c).unwrap_or(&0)).sum::<i32>();
    }
    words.sort_by(|b, a| b.score.cmp(&a.score));


        Self {
            words,
            remaining_letters: vec!['a','b','c','d','e','f','g','h','i','j','k','l','m',
                                    'n','o','p','q','r','s','t','u','v','w','x','y','z'],
            guessed_letters: vec![],
            current_word: "_ _ _ _ _".to_owned(),
            scale: 1.0,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        cc.egui_ctx.set_pixels_per_point(cc.egui_ctx.pixels_per_point()*3.0);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn clicked(&mut self, c:char) {
    if self.remaining_letters.contains(&c) {
        self.remaining_letters.retain(|&x| x != c);
        self.guessed_letters.push(c.clone());
    } else {
        return
    }

    if self.words.iter().filter(|w| !w.word.contains(c)).count() != 0 {
        self.words.retain(|w| !w.word.contains(c));

    }else {
        let mut new_words = vec![];
        for w in &self.words {
            new_words.push(w.word.chars().map(|g| match g==c {true => {g}, false => {'.'}}).collect::<String>());
        }
        
        let mut same_char_frequency = HashMap::new();
        for w in new_words {
            same_char_frequency.insert(w.clone(), same_char_frequency.get(&w).unwrap_or(&0) + 1);
        }
        let new_word = same_char_frequency.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
        self.words.retain(|w| &w.word.chars().map(|g| match g==c {true => {g}, false => {'.'}}).collect::<String>() == new_word);
    }

    self.current_word = self.words[0].word.chars().map(|g| match self.guessed_letters.contains(&g) {true => {g.to_string()+" "}, false => {"_ ".to_owned()}}).collect::<String>();

    
    }
}


impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { words, remaining_letters, guessed_letters, current_word, scale } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut clicked = None;
        let letters = self.remaining_letters.clone();
        let mut reset = false;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                ui.label("Hangman");
                if ui.button("Reset").clicked() {
                    reset = true;
                }
                if ui.small_button("-").clicked() {
                    self.scale = ctx.pixels_per_point()* 0.9;
                    ctx.set_pixels_per_point(self.scale);
                }
                if ui.small_button("+").clicked() {
                    self.scale = ctx.pixels_per_point()* 1.1;
                    ctx.set_pixels_per_point(self.scale);
                }
            });
        });

        

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical_centered(|ui| {

                    ui.heading(current_word);

                        ui.label(format!("Incorrect Letters: ({}/10)", guessed_letters.iter().filter(|c| !words[0].word.contains(**c)).count()));

                    if guessed_letters.iter().filter(|c| !words[0].word.contains(**c)).count() >= 10 {
                        ui.label("You Lost");
                        ui.label(format!("the word was {}", words[0].word.to_ascii_uppercase()));
                        ui.small(format!("you guessed: {}", guessed_letters.iter().map(|x| x.to_string()+" ").collect::<String>()));
                    }else if words[0].word.chars().all(|c| guessed_letters.contains(&c)) {
                        ui.label("You Won");
                        ui.label(format!("the word was {}", words[0].word.to_ascii_uppercase()));
                    }else {
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width()/2.0-100.0);
                            for c in "qwertyuiop".chars() {
                                if ui.add_enabled(!guessed_letters.contains(&c),egui::Button::new(c.to_string()).small()).clicked() {
                                    clicked = Some(c);
                                }
                            }
                        });
                         ui.horizontal(|ui| {
                            ui.add_space(ui.available_width()/2.0-80.0);
                            for c in "asdfghjkl".chars() {
                                if ui.add_enabled(!guessed_letters.contains(&c),egui::Button::new(c.to_string()).small()).clicked() {
                                    clicked = Some(c);
                                }
                            }
                        });
                         ui.horizontal(|ui| {
                            ui.add_space(ui.available_width()/2.0-70.0);
                            for c in "zxcvbnm".chars() {
                                if ui.add_enabled(!guessed_letters.contains(&c),egui::Button::new(c.to_string()).small()).clicked() {
                                    clicked = Some(c);
                                }
                            }
                        });
                    }
                });
        });
        if let Some(c) = clicked {
            self.clicked(c);
        }

        if reset {
            let d = Self::default();
            self.words = d.words;
            self.current_word = d.current_word;
            self.guessed_letters =d.guessed_letters;
            self.remaining_letters = d.remaining_letters;
            

        }


    }
}
