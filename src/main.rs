use eframe::egui;
use egui::{Widget, util::History};

mod apps;

trait DemoApp {
    fn app_panel(&mut self, ui: &mut egui::Ui);
    fn settings_panel(&mut self, ui: &mut egui::Ui);
}

impl DemoApp for apps::AsymCoefCmp {
    fn app_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_app_panel(ui);
    }
    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_settings_panel(ui);
    }
}

impl DemoApp for apps::Impulse {
    fn app_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_app_panel(ui);
    }
    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_settings_panel(ui);
    }
}

#[derive(Debug, PartialEq)]
enum DemoAppEnum {
    AsymCoefCmp,
    Impulse,
}

struct State {
    is_settings_panel_open: bool,
    is_force_repaint: bool,
    zoom_factor: f32,
    current_demo_app: DemoAppEnum,
    impulse_app: apps::Impulse,
    asym_coef_app: apps::AsymCoefCmp,
}

struct MainApp {
    state: State,
    frame_history: History<f32>,
}

impl MainApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let frame_history_max_age: f32 = 1.0;
        let frame_history_max_len: usize = 120;

        Self {
            state: State {
                is_settings_panel_open: true,
                is_force_repaint: false,
                zoom_factor: 1.0,
                current_demo_app: DemoAppEnum::Impulse,
                impulse_app: apps::Impulse {
                    ..Default::default()
                },
                asym_coef_app: apps::AsymCoefCmp {
                    ..Default::default()
                },
            },
            frame_history: History::new(0..frame_history_max_len, frame_history_max_age),
        }
    }

    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ctx.set_pixels_per_point(self.state.zoom_factor);
        self.top_panel(ui);
        self.settings_panel(ui);
        self.demo_app_settings_panel(ui);
        self.app_panel(ui);
    }

    fn top_panel(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.separator();
                ui.toggle_value(&mut self.state.is_settings_panel_open, "Settings");
                ui.separator();
                egui::ComboBox::from_label("Select app...")
                    .selected_text(format!("{:?}", self.state.current_demo_app))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.current_demo_app,
                            DemoAppEnum::AsymCoefCmp,
                            "AsymCoefCmp",
                        );
                        ui.selectable_value(
                            &mut self.state.current_demo_app,
                            DemoAppEnum::Impulse,
                            "Impulse",
                        );
                    });
            });
        });
    }

    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        let fps: f32 = if self.state.is_force_repaint {
            1.0 / self.frame_history.mean_time_interval().unwrap_or_default()
        } else {
            0.0
        };
        let mean_frame_time: f32 = self.frame_history.average().unwrap_or_default();

        egui::SidePanel::left("settings_panel")
            .min_width(200.0)
            .show_animated_inside(ui, self.state.is_settings_panel_open, |ui| {
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Settings");
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Zoom factor:");
                    egui::DragValue::new(&mut self.state.zoom_factor)
                        .range(0.5..=3.0)
                        .speed(0.01)
                        .max_decimals(1)
                        .ui(ui);
                });

                ui.add_space(24.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Performance");
                });
                ui.add_space(4.0);
                ui.checkbox(
                    &mut self.state.is_force_repaint,
                    "Force repaint on each frame",
                );
                ui.label(format!("FPS: {:.1}", fps));
                ui.label(format!("Frame time: {:.1} ms", 1e3 * mean_frame_time));
            });
    }

    fn demo_app_settings_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::right("demo_app_settings_panel")
            .min_width(200.0)
            .show_animated_inside(ui, self.state.is_settings_panel_open, |ui| {
                match self.state.current_demo_app {
                    DemoAppEnum::AsymCoefCmp => {
                        self.state.asym_coef_app.settings_panel(ui);
                    }
                    DemoAppEnum::Impulse => {
                        self.state.impulse_app.settings_panel(ui);
                    }
                };
            });
    }

    fn app_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_selected_app(ui);
        });
    }

    fn show_selected_app(&mut self, ui: &mut egui::Ui) {
        match self.state.current_demo_app {
            DemoAppEnum::AsymCoefCmp => {
                self.state.asym_coef_app.app_panel(ui);
            }
            DemoAppEnum::Impulse => {
                self.state.impulse_app.app_panel(ui);
            }
        };
    }

    fn add_frame_to_history(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now: f64 = ctx.input(|i| i.time);
        let previous_frame_time: f32 = (frame.info().cpu_usage).unwrap_or_default();
        self.frame_history.add(now, previous_frame_time);
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let cp_frame = egui::Frame::central_panel(&ctx.style()).inner_margin(0.0);
        egui::CentralPanel::default()
            .frame(cp_frame)
            .show(ctx, |ui| {
                self.add_frame_to_history(ctx, frame);
                self.ui(ctx, ui);
                if self.state.is_force_repaint {
                    ctx.request_repaint()
                };
            });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Погружатель",
        native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))),
    )
}
