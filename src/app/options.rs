pub struct Options {
    pub harmonic_kernel_size: usize,
    pub percussive_kernel_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            harmonic_kernel_size: 20,
            percussive_kernel_size: 20,
        }
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Options");
        ui.add(
            egui::Slider::new(&mut self.harmonic_kernel_size, 1..=100).text("Harmonic Kernel Size"),
        );
        ui.add(
            egui::Slider::new(&mut self.percussive_kernel_size, 1..=100)
                .text("Percussive Kernel Size"),
        );
    }
}
