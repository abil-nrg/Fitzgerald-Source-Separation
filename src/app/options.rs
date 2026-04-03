use crate::algorithm::{WindowFunction, hann_window, rectangular_window};

pub struct Options {
    pub harmonic_kernel_size: usize,
    pub percussive_kernel_size: usize,
    pub window_function: WindowFunction,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            harmonic_kernel_size: 20,
            percussive_kernel_size: 20,
            window_function: WindowFunction::HannWindow,
        }
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Options");

        egui::ComboBox::from_label("Window Function")
            .selected_text(self.window_function.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.window_function,
                    WindowFunction::HannWindow,
                    WindowFunction::HannWindow.name(),
                );
                ui.selectable_value(
                    &mut self.window_function,
                    WindowFunction::RectangularWindow,
                    WindowFunction::RectangularWindow.name(),
                );
            });

        ui.add(
            egui::Slider::new(&mut self.harmonic_kernel_size, 1..=100).text("Harmonic Kernel Size"),
        );
        ui.add(
            egui::Slider::new(&mut self.percussive_kernel_size, 1..=100)
                .text("Percussive Kernel Size"),
        );
    }
}
