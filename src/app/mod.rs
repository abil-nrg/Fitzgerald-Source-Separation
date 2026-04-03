mod audio;
pub mod menu_bar;
mod options;
pub mod spectrogram;

use audio::Audio;

#[derive(Default)]
pub struct SeparationApp {
    menu_bar: menu_bar::MenuBar,
    current_stream: Option<cpal::Stream>,

    original: Option<Audio>,
    harmonic: Option<Audio>,
    percussive: Option<Audio>,

    options: options::Options,
}

impl SeparationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for SeparationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(bytes) = self.menu_bar.poll_file() {
            match fitzgerald_source_separation::audio::load_audio_from_bytes(bytes) {
                Ok(data) => {
                    log::info!(
                        "loaded audio: {} samples, {} channels",
                        data.samples.len(),
                        data.channels
                    );

                    let original = Audio::from_audio_data(ctx, data);
                    self.original = Some(original);
                }
                Err(e) => log::error!("can't load audio: {e}"),
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar.draw(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Source Separation");
            if let Some(original) = &self.original {
                ui.label(format!("Sample rate: {} Hz", original.data.sample_rate));
                if ui.button("Play Original").clicked() {
                    match fitzgerald_source_separation::audio::play_audio(&original.data) {
                        Ok(stream) => self.current_stream = Some(stream),
                        Err(e) => log::error!("playback failed: {}", e),
                    }
                }

                if let (Some(harmonic), Some(percussive)) = (&self.harmonic, &self.percussive) {
                    if ui.button("Play Harmonic").clicked() {
                        match fitzgerald_source_separation::audio::play_audio(&harmonic.data) {
                            Ok(stream) => self.current_stream = Some(stream),
                            Err(e) => log::error!("playback failed: {}", e),
                        }
                    }

                    if ui.button("Play Percussive").clicked() {
                        match fitzgerald_source_separation::audio::play_audio(&percussive.data) {
                            Ok(stream) => self.current_stream = Some(stream),
                            Err(e) => log::error!("playback failed: {}", e),
                        }
                    }
                }

                if ui.button("Stop Audio").clicked() {
                    self.current_stream = None;
                }

                ui.separator();

                self.options.ui(ui);

                if ui.button("Separate!").clicked() {
                    let (harmonic, percussive) = original.separate(
                        ctx,
                        self.options.harmonic_kernel_size,
                        self.options.percussive_kernel_size,
                    );
                    self.harmonic = Some(harmonic);
                    self.percussive = Some(percussive);
                }
            } else {
                ui.label("Load an audio file");
            }
        });

        if let Some(audio) = &self.original {
            egui::Window::new("Original Spectrogram").show(ctx, |ui| {
                audio.spectrogram.ui(ui);
            });
        }

        if let Some(audio) = &self.harmonic {
            egui::Window::new("Harmonic Spectrogram").show(ctx, |ui| {
                audio.spectrogram.ui(ui);
            });
        }

        if let Some(audio) = &self.percussive {
            egui::Window::new("Percussive Spectrogram").show(ctx, |ui| {
                audio.spectrogram.ui(ui);
            });
        }
    }
}
