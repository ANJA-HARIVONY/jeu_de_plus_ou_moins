#![windows_subsystem = "windows"]

use eframe::egui;
use rand::Rng;
use std::cmp::Ordering;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Jeu de Plus ou Moins",
        options,
        Box::new(|_cc| Ok(Box::new(JeuPlusOuMoins::new()))),
    )
}

struct JeuPlusOuMoins {
    secret_number: u32,
    input_text: String,
    message: String,
    attempts: u32,
    game_over: bool,
    history: Vec<(u32, String)>,
}

impl Default for JeuPlusOuMoins {
    fn default() -> Self {
        Self::new()
    }
}

impl JeuPlusOuMoins {
    fn new() -> Self {
        Self {
            secret_number: rand::thread_rng().gen_range(1..=100),
            input_text: String::new(),
            message: String::from("Devinez le nombre entre 1 et 100 !"),
            attempts: 0,
            game_over: false,
            history: Vec::new(),
        }
    }

    fn reset_game(&mut self) {
        self.secret_number = rand::thread_rng().gen_range(1..=100);
        self.input_text.clear();
        self.message = String::from("Devinez le nombre entre 1 et 100 !");
        self.attempts = 0;
        self.game_over = false;
        self.history.clear();
    }

    fn make_guess(&mut self) {
        if let Ok(guess) = self.input_text.trim().parse::<u32>() {
            if guess < 1 || guess > 100 {
                self.message = String::from("⚠️ Veuillez entrer un nombre entre 1 et 100 !");
                return;
            }

            self.attempts += 1;

            match guess.cmp(&self.secret_number) {
                Ordering::Less => {
                    self.message = format!("📈 Plus ! (Tentative {})", self.attempts);
                    self.history.push((guess, "Plus !".to_string()));
                }
                Ordering::Greater => {
                    self.message = format!("📉 Moins ! (Tentative {})", self.attempts);
                    self.history.push((guess, "Moins !".to_string()));
                }
                Ordering::Equal => {
                    self.message = format!("🎉 Gagné en {} tentative(s) !", self.attempts);
                    self.history.push((guess, "Gagné ! 🎉".to_string()));
                    self.game_over = true;
                }
            }
            self.input_text.clear();
        } else {
            self.message = String::from("⚠️ Veuillez entrer un nombre valide !");
        }
    }
}

impl eframe::App for JeuPlusOuMoins {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("🎲 Jeu de Plus ou Moins");
                ui.add_space(10.0);
            });

            ui.separator();
            ui.add_space(10.0);

            // Message principal
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(&self.message)
                        .size(18.0)
                        .color(if self.game_over {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::WHITE
                        }),
                );
            });

            ui.add_space(20.0);

            if !self.game_over {
                ui.horizontal(|ui| {
                    ui.label("Votre supposition : ");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.input_text)
                            .desired_width(100.0)
                            .hint_text("1-100"),
                    );

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.make_guess();
                    }

                    if ui.button("✓ Valider").clicked() {
                        self.make_guess();
                    }
                });
            }

            ui.add_space(20.0);

            // Bouton Nouvelle Partie
            ui.horizontal(|ui| {
                if ui.button("🔄 Nouvelle Partie").clicked() {
                    self.reset_game();
                }

                ui.label(format!("Tentatives : {}", self.attempts));
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Historique
            if !self.history.is_empty() {
                ui.label(egui::RichText::new("📜 Historique").size(16.0).strong());
                ui.add_space(5.0);

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (i, (guess, result)) in self.history.iter().enumerate().rev() {
                            ui.horizontal(|ui| {
                                ui.label(format!("#{}", self.history.len() - i));
                                ui.label(format!("Supposition : {}", guess));
                                ui.label(format!("→ {}", result));
                            });
                        }
                    });
            }

            ui.add_space(10.0);
            ui.separator();
            
            // Instructions
            ui.collapsing("ℹ️ Comment jouer", |ui| {
                ui.label("• L'ordinateur choisit un nombre entre 1 et 100");
                ui.label("• Entrez votre supposition");
                ui.label("• Le jeu vous indique si c'est Plus ou Moins");
                ui.label("• Trouvez le nombre en un minimum de tentatives !");
            });
        });
    }
}
