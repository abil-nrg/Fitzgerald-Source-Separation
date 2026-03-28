use fitzgerald_source_separation::audio::AudioData;

use crate::algorithm::stft;

pub mod menu_bar;

const WINDOW_SIZE: usize = 1024 * 2;
const HOP_SIZE: usize = 512 * 2;
const SPECTROGRAM_HEIGHT: usize = 1024;

pub struct Spectrogram {
    texture: egui::TextureHandle,
}

impl Spectrogram {
    pub fn from_audio(ctx: &egui::Context, audio: &AudioData) -> Self {
        let mono = audio.to_mono();
        let frames = stft::stft(&mono, WINDOW_SIZE, HOP_SIZE);

        let num_bins = WINDOW_SIZE / 2 + 1;
        let num_frames = frames.len();

        let db_matrix: Vec<Vec<f64>> = frames
            .iter()
            .map(|frame| {
                frame[..num_bins]
                    .iter()
                    .map(|c| 20.0 * (c.norm() + f64::EPSILON).log10())
                    .collect()
            })
            .collect();

        let max_db = db_matrix
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let min_db = max_db - 80.0;
        let db_range = max_db - min_db;

        let w = num_frames;
        let h = SPECTROGRAM_HEIGHT;
        let mut pixels = vec![egui::Color32::BLACK; w * h];

        for px in 0..w {
            let frame = &db_matrix[px];
            for py in 0..h {
                let bin_f = (1.0 - py as f64 / h as f64) * (num_bins - 1) as f64;
                let bin = bin_f as usize;
                let db = frame[bin.min(num_bins - 1)];
                let normalized = ((db - min_db) / db_range).clamp(0.0, 1.0);
                let v = (normalized * 255.0) as u8;
                pixels[py * w + px] = egui::Color32::from_rgb(v, v, v);
            }
        }

        let texture = egui::ColorImage::new([w, h], pixels);

        Self {
            texture: ctx.load_texture("spectrogram", texture, egui::TextureOptions::LINEAR),
        }
    }
}

#[derive(Default)]
pub struct SeparationApp {
    menu_bar: menu_bar::MenuBar,
    loaded_audio: Option<AudioData>,
    current_stream: Option<cpal::Stream>,
    spectrogram: Option<Spectrogram>,
}
fn ui_spectrogram(ui: &mut egui::Ui, spectrogram: &Spectrogram) {
    let image = egui::Image::new(&spectrogram.texture)
        .shrink_to_fit()
        .maintain_aspect_ratio(false);
    ui.add(image);
}

impl SeparationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SeparationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(bytes) = self.menu_bar.poll_file() {
            match fitzgerald_source_separation::audio::load_audio_from_bytes(bytes) {
                Ok(data) => {
                    log::info!("loaded audio: {} samples", data.samples.len());
                    self.spectrogram = Some(Spectrogram::from_audio(ctx, &data));
                    self.loaded_audio = Some(data);
                }
                Err(e) => log::error!("can't load audio: {e}"),
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar.draw(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Source Separation");

            if let Some(audio) = &self.loaded_audio {
                if ui.button("Play original").clicked() {
                    match fitzgerald_source_separation::audio::play_audio(audio) {
                        Ok(stream) => self.current_stream = Some(stream),
                        Err(e) => log::error!("playback failed: {}", e),
                    }
                }

                if ui.button("Stop").clicked() {
                    self.current_stream = None;
                }

                ui.label(format!("Sample rate: {} Hz", audio.sample_rate));

                if let Some(spectrogram) = &self.spectrogram {
                    ui_spectrogram(ui, spectrogram);
                }
            } else {
                ui.label("load an audio file.");
            }
        });
    }
}
