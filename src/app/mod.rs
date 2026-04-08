mod audio;
mod menu_bar;
mod options;
mod spectrogram;

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
        Self::default()
    }
    fn ui_waveform(&self, ui: &mut egui::Ui, audio_data: &fitzgerald_source_separation::audio::AudioData){
        let desired_size = egui::vec2(ui.available_width(), 80.0);
        let (rect, _response) = ui.allocate_at_least(desired_size, egui::Sense::hover());
        
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect,2.0,ui.visuals().extreme_bg_color);

        let mono = audio_data.to_mono();
        let middle_y = rect.center().y;
        let width = rect.width();
        let height = rect.height();

        let step = (mono.len() as f32/width).max(1.0) as usize;

        let points: Vec<egui::Pos2> = mono
            .iter()
            .step_by(step)
            .enumerate()
            .map(|(x,&sample)| {
                let x_pos = rect.left() + x as f32;
                let y_offset = sample*(height/2.0)*0.9;
                egui::pos2(x_pos, middle_y-y_offset)
            })
            .collect();
        
        if points.len() > 1 {
            painter.add(egui::Shape::line(
                points,
                egui::Stroke::new(1.0, ui.visuals().widgets.active.fg_stroke.color),
            ));
        }
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

                    let original = Audio::from_audio_data(
                        ctx,
                        data,
                        self.options.window_function.fun(),
                        self.options.fft_window_size,
                        self.options.fft_hop_size,
                    );
                    self.original = Some(original);
                    self.harmonic = None;
                    self.percussive = None;
                    self.current_stream = None;
                }
                Err(e) => log::error!("can't load audio: {e}"),
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar.draw(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Source Separation");
            if self.original.is_none() {
                ui.label("Change settings then load an audio file");
            }
            ui.separator();
            self.options.ui(ui);
            ui.separator();
            if let Some(original) = &self.original {
                ui.label(format!("Sample rate: {} Hz", original.data.sample_rate));

                if ui.button("Stop Audio").clicked() {
                    self.current_stream = None;
                }

                ui.separator();

                if ui.button("Separate!").clicked() {
                    let (harmonic, percussive) = original.separate(
                        ctx,
                        self.options.harmonic_kernel_size,
                        self.options.percussive_kernel_size,
                        self.options.window_function.fun(),
                        self.options.fft_window_size,
                        self.options.fft_hop_size,
                    );
                    self.harmonic = Some(harmonic);
                    self.percussive = Some(percussive);
                }
            } else {
                ui.label("For more information, see the GitHub Repository:");
                ui.hyperlink("https://github.com/abil-nrg/Fitzgerald-Source-Separation");
            }
        });


        if self.original.is_some() {
            egui::Window::new("Original").show(ctx, |ui| {
                let audio = self.original.as_ref().unwrap();
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        match fitzgerald_source_separation::audio::play_audio(&audio.data) {
                            Ok(stream) => self.current_stream = Some(stream),
                            Err(e) => log::error!("playback failed: {e}"),
                        }
                    }
                });
                ui.label("Waveform:");
                self.ui_waveform(ui, &audio.data);
                ui.separator();
                ui.label("Spectrogram:");
                audio.spectrogram.ui(ui);
            });
        }

        if self.harmonic.is_some() {
            egui::Window::new("Harmonic").show(ctx, |ui| {
                let audio = self.harmonic.as_ref().unwrap();
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        match fitzgerald_source_separation::audio::play_audio(&audio.data) {
                            Ok(stream) => self.current_stream = Some(stream),
                            Err(e) => log::error!("playback failed: {e}"),
                        }
                    }
                    if ui.button("Save WAV").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let _ = fitzgerald_source_separation::audio::save_wav(
                                &path.to_string_lossy(), 
                                &audio.data
                            );
                        }
                    }
                });
                ui.label("Waveform:");
                self.ui_waveform(ui, &audio.data);
                ui.separator();
                ui.label("Spectrogram:");
                audio.spectrogram.ui(ui);
            });
        }

        if self.percussive.is_some() {
            egui::Window::new("Percussive").show(ctx, |ui| {
                let audio = self.percussive.as_ref().unwrap();
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        match fitzgerald_source_separation::audio::play_audio(&audio.data) {
                            Ok(stream) => self.current_stream = Some(stream),
                            Err(e) => log::error!("playback failed: {e}"),
                        }
                    }
                    if ui.button("Save WAV").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let _ = fitzgerald_source_separation::audio::save_wav(
                                &path.to_string_lossy(), 
                                &audio.data
                            );
                        }
                    }
                });
                ui.label("Waveform:");
                self.ui_waveform(ui, &audio.data);
                ui.separator();
                ui.label("Spectrogram:");
                audio.spectrogram.ui(ui);
            });
        }
    }
}
