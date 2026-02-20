use eframe::egui;

pub struct AsymCoefCmp {}

impl Default for AsymCoefCmp {
    fn default() -> Self {
        Self {}
    }
}

impl AsymCoefCmp {
    pub fn draw_app_panel(&mut self, _ui: &mut egui::Ui) {}

    pub fn draw_settings_panel(&mut self, _ui: &mut egui::Ui) {}
}
