use eframe::egui;
use egui::{Widget, util::History};

#[cfg(target_arch = "wasm32")]
use eframe::web_sys;

mod apps;

trait MiniApp {
    fn app_panel(&mut self, ui: &mut egui::Ui);
    fn settings_panel(&mut self, ui: &mut egui::Ui);
}

impl MiniApp for apps::AsymCoefCmp {
    fn app_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_app_panel(ui);
    }
    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_settings_panel(ui);
    }
}

impl MiniApp for apps::ImpulseCmp {
    fn app_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_app_panel(ui);
    }
    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        self.draw_settings_panel(ui);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CurrentMiniApp {
    AsymCoefCmp,
    ImpulseCmp,
}

struct State {
    is_settings_panel_open: bool,
    is_force_repaint: bool,
    zoom_factor: f32,
    current_mini_app: CurrentMiniApp,
    impulse_cmp: apps::ImpulseCmp,
    asym_coef_cmp: apps::AsymCoefCmp,
}

struct MainApp {
    frame_history: History<f32>,
    state: State,
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
                current_mini_app: CurrentMiniApp::ImpulseCmp,
                impulse_cmp: apps::ImpulseCmp {
                    ..Default::default()
                },
                asym_coef_cmp: apps::AsymCoefCmp {
                    ..Default::default()
                },
            },
            frame_history: History::new(0..frame_history_max_len, frame_history_max_age),
        }
    }

    pub fn mini_apps(
        &mut self,
    ) -> impl Iterator<Item = (&'static str, CurrentMiniApp, &mut dyn MiniApp)> {
        let vec = vec![
            (
                "Импульс",
                CurrentMiniApp::ImpulseCmp,
                &mut self.state.impulse_cmp as &mut dyn MiniApp,
            ),
            (
                "Коэффициент асимметрии",
                CurrentMiniApp::AsymCoefCmp,
                &mut self.state.asym_coef_cmp as &mut dyn MiniApp,
            ),
        ];
        vec.into_iter()
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
                ui.toggle_value(&mut self.state.is_settings_panel_open, "Настройки");
                ui.separator();
                egui::ComboBox::from_label("")
                    .selected_text({
                        let current_mini_app = self.state.current_mini_app;
                        let mut text = "";
                        for (name, mini_app_enum, _mini_app_instance) in self.mini_apps() {
                            if current_mini_app == mini_app_enum {
                                text = name;
                            }
                        }
                        text
                    })
                    .show_ui(ui, |ui| {
                        let mut current_mini_app = self.state.current_mini_app;
                        for (name, mini_app_enum, _mini_app_instance) in self.mini_apps() {
                            ui.selectable_value(&mut current_mini_app, mini_app_enum, name);
                        }
                        self.state.current_mini_app = current_mini_app;
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
                let current_mini_app = self.state.current_mini_app;
                for (_name, app, mini_app) in self.mini_apps() {
                    if app == current_mini_app {
                        mini_app.settings_panel(ui);
                    }
                }
            });
    }

    fn app_panel(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_selected_app(ui);
        });
    }

    fn show_selected_app(&mut self, ui: &mut egui::Ui) {
        let current_mini_app = self.state.current_mini_app;
        for (_name, app, mini_app) in self.mini_apps() {
            if app == current_mini_app {
                mini_app.app_panel(ui);
            }
        }
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("canvas")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("canvas was not a HtmlCanvasElement");

        let _ = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(MainApp::new(cc)))),
            )
            .await;
    });
}
