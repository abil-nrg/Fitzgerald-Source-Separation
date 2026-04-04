use crate::algorithm::Window;

pub struct Options {
    pub harmonic_kernel_size: usize,
    pub percussive_kernel_size: usize,
    pub window_function: Window,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            harmonic_kernel_size: 20,
            percussive_kernel_size: 20,
            window_function: Window::Hann,
        }
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Options");

        egui::ComboBox::from_label("Window Function")
            .selected_text(self.window_function.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.window_function, Window::Hann, Window::Hann.name());
                ui.selectable_value(
                    &mut self.window_function,
                    Window::Rectangular,
                    Window::Rectangular.name(),
                );
                ui.selectable_value(
                    &mut self.window_function,
                    Window::Triangular,
                    Window::Triangular.name(),
                );
                ui.selectable_value(
                    &mut self.window_function,
                    Window::Blackman,
                    Window::Blackman.name(),
                );
                ui.selectable_value(
                    &mut self.window_function,
                    Window::Hamming,
                    Window::Hamming.name(),
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
