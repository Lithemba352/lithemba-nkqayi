use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Legends of Hokage Lilz",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    health: i32,
    inventory: Vec<String>,
    game_over: bool,
    game_won: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            health: 100,
            inventory: vec!["Ninja Scroll".to_string()],
            game_over: false,
            game_won: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Legends of Hokage Lilz");

            if self.game_over {
                ui.label("GAME OVER!");
                return;
            }

            if self.game_won {
                ui.label("Congratulations! YOU WIN!");
                return;
            }

            ui.label(format!("Health: {}", self.health));
            ui.label("Inventory:");
            for item in &self.inventory {
                ui.label(item);
            }

            ui.separator();

            if ui.button("Go Left").clicked() {
                self.inventory.push("Mysterious Potion".to_string());
            }

            if ui.button("Go Right").clicked() {
                self.health -= 30;
                if self.health <= 0 {
                    self.game_over = true;
                }
            }

            if ui.button("Check Inventory").clicked() {
                // Do nothing, just re-render
            }

            if ui.button("Quit").clicked() {
                self.game_over = true;
            }
        });
    }
}