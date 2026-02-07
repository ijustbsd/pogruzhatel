use eframe::egui;

pub struct Impulse {}

impl Default for Impulse {
    fn default() -> Self {
        Self {}
    }
}

impl Impulse {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Hello from Impulse");
    }
}
