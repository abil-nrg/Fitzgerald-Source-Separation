use crate::algorithm::Window;

pub struct Options {
    pub harmonic_kernel_size: usize,
    pub percussive_kernel_size: usize,
    pub window_function: Window,
    pub fft_window_size: usize,
    pub fft_hop_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            harmonic_kernel_size: 20,
            percussive_kernel_size: 20,
            window_function: Window::Hann,
            fft_window_size: 2048,
            fft_hop_size: 1024,
        }
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Options");
        ui.label("note: changing certain options requires audio to be reloaded");

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
        egui::ComboBox::from_label("FFT Window Size")
            .selected_text(format!("{}", self.fft_window_size))
            .show_ui(ui, |ui| {
                for &size in &[64, 128, 256, 512, 1024, 2048, 4096, 8192] {
                    ui.selectable_value(&mut self.fft_window_size, size, format!("{size}"));
                }
            });
        egui::ComboBox::from_label("FFT Hop Size")
            .selected_text(format!("{}", self.fft_hop_size))
            .show_ui(ui, |ui| {
                for &size in &[32, 64, 128, 256, 512, 1024, 2048, 4096] {
                    ui.selectable_value(&mut self.fft_hop_size, size, format!("{size}"));
                }
            });
    }
}
