use eframe::egui;

pub struct AsymCoefCmp {}

impl Default for AsymCoefCmp {
    fn default() -> Self {
        Self {}
    }
}

impl AsymCoefCmp {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Hello from AsymCoefCmp");
    }
}
