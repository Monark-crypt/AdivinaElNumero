use eframe::egui;
use rand::Rng;
use std::cmp::Ordering;

struct GuessApp {
    secret_number: u32,
    guess_input: String,
    message: String,
    game_over: bool,
}

impl Default for GuessApp {
    fn default() -> Self {
        Self {
            secret_number: rand::thread_rng().gen_range(1, 101),
            guess_input: String::new(),
            message: "Guess a number between 1 and 100".to_string(),
            game_over: false,
        }
    }
}

impl eframe::App for GuessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(" Guess the Number");
            ui.add_space(10.0);

            ui.label(&self.message);

            ui.add_space(10.0);

            ui.text_edit_singleline(&mut self.guess_input);

            if ui.button("Guess").clicked() && !self.game_over {
                if let Ok(guess) = self.guess_input.trim().parse::<u32>() {
                    match guess.cmp(&self.secret_number) {
                        Ordering::Less => self.message = "Too small!".to_string(),
                        Ordering::Greater => self.message = "Too big!".to_string(),
                        Ordering::Equal => {
                            self.message = "You Win!".to_string();
                            self.game_over = true;
                        }
                    }
                } else {
                    self.message = "Please enter a number".to_string();
                }
                self.guess_input.clear();
            }

            if self.game_over {
                ui.add_space(10.0);
                if ui.button("Restart").clicked() {
                    *self = Self::default();
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guess the Number",
        options,
        Box::new(|_cc| Box::new(GuessApp::default())),
    )
}



// comentario
/*
comentarios en bloque
 */