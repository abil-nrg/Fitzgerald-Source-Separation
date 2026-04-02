use fitzgerald_source_separation::audio::AudioData;
use num::Complex;

use crate::{
    algorithm::{filter, stft},
    app::spectrogram::Spectrogram,
};

pub mod menu_bar;
pub mod spectrogram;

#[derive(Default)]
pub struct SeparationApp {
    menu_bar: menu_bar::MenuBar,
    loaded_audio: Option<AudioData>,
    current_stream: Option<cpal::Stream>,

    original_spectrogram: Option<Spectrogram>,
    original_frames: Option<Vec<Vec<Complex<f64>>>>,

    harmonic_frames: Option<Vec<Vec<Complex<f64>>>>,
    harmonic_spectrogram: Option<Spectrogram>,
    harmonic_audio: Option<AudioData>,

    percussive_frames: Option<Vec<Vec<Complex<f64>>>>,
    percussive_spectrogram: Option<Spectrogram>,
    percussive_audio: Option<AudioData>,
}

impl SeparationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn separate(&mut self) {
        const HARMONIC_KERNEL: usize = 17; // median filter along time axis
        const PERCUSSIVE_KERNEL: usize = 17; // median filter along frequency axis

        let frames = match &self.original_frames {
            Some(f) => f,
            None => return,
        };

        let num_frames = frames.len();
        let num_bins = frames[0].len();

        let mag: Vec<Vec<f64>> = frames
            .iter()
            .map(|frame| frame.iter().map(|c| c.norm()).collect())
            .collect();

        let mut h_mag = vec![vec![0.0f64; num_bins]; num_frames];
        for f in 0..num_bins {
            let time_slice: Vec<f64> = (0..num_frames).map(|t| mag[t][f]).collect();
            let filtered = filter::median_filter(&time_slice, HARMONIC_KERNEL);
            for t in 0..num_frames {
                h_mag[t][f] = filtered[t];
            }
        }

        let mut p_mag = vec![vec![0.0f64; num_bins]; num_frames];
        for t in 0..num_frames {
            p_mag[t] = filter::median_filter(&mag[t], PERCUSSIVE_KERNEL);
        }

        let mut harmonic_frames = vec![vec![Complex::new(0.0, 0.0); num_bins]; num_frames];
        let mut percussive_frames = vec![vec![Complex::new(0.0, 0.0); num_bins]; num_frames];

        for t in 0..num_frames {
            for f in 0..num_bins {
                if h_mag[t][f] >= p_mag[t][f] {
                    harmonic_frames[t][f] = frames[t][f];
                } else {
                    percussive_frames[t][f] = frames[t][f];
                }
            }
        }

        self.harmonic_frames = Some(harmonic_frames);
        self.percussive_frames = Some(percussive_frames);
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

                    let mono = if data.channels == 1 {
                        data.samples.clone()
                    } else {
                        data.to_mono()
                    };
                    let frames = stft::stft(&mono, spectrogram::WINDOW_SIZE, spectrogram::HOP_SIZE);
                    self.original_spectrogram = Some(Spectrogram::from_audio(ctx, &frames));
                    self.loaded_audio = Some(if data.channels == 1 {
                        AudioData {
                            samples: data.samples.iter().flat_map(|&s| [s, s]).collect(),
                            sample_rate: data.sample_rate,
                            channels: 2,
                        }
                    } else {
                        data
                    });
                    self.original_frames = Some(frames);
                    self.separate();

                    let sample_rate = self.loaded_audio.as_ref().unwrap().sample_rate;
                    let output_length = mono.len();

                    self.harmonic_spectrogram = Some(Spectrogram::from_audio(
                        ctx,
                        self.harmonic_frames.as_ref().unwrap(),
                    ));
                    self.harmonic_audio = Some(AudioData {
                        samples: stft::istft(
                            self.harmonic_frames.as_ref().unwrap(),
                            spectrogram::WINDOW_SIZE,
                            spectrogram::HOP_SIZE,
                            output_length,
                        )
                        .into_iter()
                        .flat_map(|s| [s, s])
                        .collect(),
                        sample_rate,
                        channels: 2,
                    });

                    self.percussive_spectrogram = Some(Spectrogram::from_audio(
                        ctx,
                        self.percussive_frames.as_ref().unwrap(),
                    ));
                    self.percussive_audio = Some(AudioData {
                        samples: stft::istft(
                            self.percussive_frames.as_ref().unwrap(),
                            spectrogram::WINDOW_SIZE,
                            spectrogram::HOP_SIZE,
                            output_length,
                        )
                        .into_iter()
                        .flat_map(|s| [s, s])
                        .collect(),
                        sample_rate,
                        channels: 2,
                    });
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

                if let Some(harmonic) = &self.harmonic_audio
                    && ui.button("Play harmonic").clicked()
                {
                    match fitzgerald_source_separation::audio::play_audio(harmonic) {
                        Ok(stream) => self.current_stream = Some(stream),
                        Err(e) => log::error!("playback failed: {}", e),
                    }
                }

                if let Some(percussive) = &self.percussive_audio
                    && ui.button("Play percussive").clicked()
                {
                    match fitzgerald_source_separation::audio::play_audio(percussive) {
                        Ok(stream) => self.current_stream = Some(stream),
                        Err(e) => log::error!("playback failed: {}", e),
                    }
                }

                if ui.button("Stop").clicked() {
                    self.current_stream = None;
                }

                ui.label(format!("Sample rate: {} Hz", audio.sample_rate));
            } else {
                ui.label("load an audio file.");
            }
        });

        if let Some(spectrogram) = &self.original_spectrogram {
            egui::Window::new("Original Spectrogram").show(ctx, |ui| {
                spectrogram.draw(ui);
            });
        }

        if let Some(spectrogram) = &self.harmonic_spectrogram {
            egui::Window::new("Harmonic Spectrogram").show(ctx, |ui| {
                spectrogram.draw(ui);
            });
        }

        if let Some(spectrogram) = &self.percussive_spectrogram {
            egui::Window::new("Percussive Spectrogram").show(ctx, |ui| {
                spectrogram.draw(ui);
            });
        }
    }
}
