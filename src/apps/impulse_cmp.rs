use eframe::egui;
use eframe::egui::Color32;
use egui::Widget;
use egui_plot::AxisHints;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::PlotPoints;
use std::f64::consts::PI;

pub struct State {
    pub vibration_points: Vec<PlotPoint>,
    pub impulse_points: Vec<PlotPoint>,
    pub debalances_pairs_count: i64,
}

pub struct ImpulseCmp {
    pub state: State,
    pub debalances_pairs_count: u8,
}

impl Default for ImpulseCmp {
    fn default() -> Self {
        Self {
            state: State {
                vibration_points: vec![],
                impulse_points: vec![],
                debalances_pairs_count: 6,
            },
            debalances_pairs_count: 6,
        }
    }
}

impl ImpulseCmp {
    fn vibration_function(&self, t: Vec<f64>, n: i64, m: f64, r: f64, omega: f64) -> Vec<f64> {
        let mut result = vec![0.0; t.len()];
        for i in 0..t.len() {
            result[i] = (n as f64) * m * omega.powi(2) * r * (omega * t[i]).cos()
        }
        return result;
    }

    fn impulse_function(
        &self,
        t: Vec<f64>,
        n: i64,
        m: Vec<f64>,
        r: Vec<f64>,
        omega_0: f64,
    ) -> Vec<f64> {
        let mut omega = vec![0.0; n as usize];
        for i in 0..omega.len() {
            omega[i] = omega_0 * (i + 1) as f64;
        }
        let mut result = vec![0.0; t.len()];
        for i in 0..t.len() {
            let mut sum = 0.0;
            for j in 0..n {
                sum += m[j as usize]
                    * omega[j as usize].powi(2)
                    * r[j as usize]
                    * (omega[j as usize] * t[i]).cos();
            }
            result[i] = sum;
        }
        return result;
    }

    fn calculate(&mut self) {
        let m = vec![
            2.75758026171761,
            0.969494952543874,
            0.486348994233291,
            0.273755006621712,
            0.155229853500278,
            0.076567059516108,
        ];
        let r = vec![
            0.020070401444444,
            0.011900487555556,
            0.008428804666667,
            0.006323725555556,
            0.004761892666667,
            0.003344359555556,
        ];

        let mut t_points = vec![0.0; 1001];
        let dt = 2.0 * PI / (t_points.len() - 1) as f64;
        for i in 0..t_points.len() {
            t_points[i] = -PI + (dt * i as f64);
        }

        self.state.debalances_pairs_count = self.debalances_pairs_count as i64;
        let vibration_points = self.vibration_function(t_points.clone(), 1, m[0], r[0], 1.0);
        let impulse_points = self.impulse_function(
            t_points.clone(),
            self.state.debalances_pairs_count,
            m,
            r,
            1.0,
        );

        self.state.vibration_points = (0..t_points.len())
            .map(|i| PlotPoint::new(t_points[i], vibration_points[i]))
            .collect();
        self.state.impulse_points = (0..t_points.len())
            .map(|i| PlotPoint::new(t_points[i], impulse_points[i]))
            .collect();
    }

    pub fn draw_app_panel(&mut self, ui: &mut egui::Ui) {
        let x_axes = vec![AxisHints::new_x().label("Время")];
        let y_axes = vec![AxisHints::new_y().label("Сила")];
        Plot::new("x_plot")
            .legend(Legend::default())
            .custom_x_axes(x_axes)
            .custom_y_axes(y_axes)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(
                        "vibration_line",
                        PlotPoints::Borrowed(&self.state.vibration_points),
                    )
                    .name("N = 1")
                    .color(Color32::from_rgb(255, 0, 0)),
                );
                plot_ui.line(
                    Line::new(
                        "impulse_line",
                        PlotPoints::Borrowed(&self.state.impulse_points),
                    )
                    .name(format!("N = {}", self.state.debalances_pairs_count))
                    .color(Color32::from_rgb(0, 255, 0)),
                );
            })
            .response;
    }

    pub fn draw_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.heading("Настройки модели");
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Количество пар дебалансов:");
            egui::DragValue::new(&mut self.debalances_pairs_count)
                .range(1..=6)
                .speed(0.1)
                .ui(ui);
        });
        ui.add_space(24.0);
        ui.vertical_centered(|ui| {
            if ui.button("Построить график!").clicked() {
                self.calculate();
            };
        });
    }
}
