use fitzgerald_source_separation::audio::AudioData;
use num::Complex;

use crate::{
    algorithm::{filter, stft},
    app::spectrogram::{self, Spectrogram},
};

pub struct Audio {
    pub data: AudioData,
    pub spectrogram: Spectrogram,
    pub frames: Vec<Vec<Complex<f64>>>,
}

impl Audio {
    pub fn from_audio_data(ctx: &egui::Context, data: AudioData) -> Self {
        let mono = if data.channels == 1 {
            data.samples.clone()
        } else {
            data.to_mono()
        };
        let frames = stft::stft(&mono, spectrogram::WINDOW_SIZE, spectrogram::HOP_SIZE);
        let spectrogram = Spectrogram::from_audio(ctx, &frames);
        Self {
            data,
            spectrogram,
            frames,
        }
    }

    pub fn from_frames(
        ctx: &egui::Context,
        frames: Vec<Vec<Complex<f64>>>,
        sample_rate: u32,
        channels: usize,
    ) -> Self {
        let spectrogram = Spectrogram::from_audio(ctx, &frames);
        let total_len = (frames.len() - 1) * spectrogram::HOP_SIZE + spectrogram::WINDOW_SIZE;
        let samples = stft::istft(
            &frames,
            spectrogram::WINDOW_SIZE,
            spectrogram::HOP_SIZE,
            total_len,
        );
        let data = AudioData {
            samples,
            sample_rate,
            channels,
        };
        Self {
            data,
            spectrogram,
            frames,
        }
    }

    pub fn separate(
        &self,
        ctx: &egui::Context,
        harmonic_kernel_size: usize,
        percussive_kernel_size: usize,
    ) -> (Audio, Audio) {
        let num_frames = self.frames.len();
        let num_bins = self.frames[0].len();

        let mag: Vec<Vec<f64>> = self
            .frames
            .iter()
            .map(|frame| frame.iter().map(|c| c.norm()).collect())
            .collect();

        let mut h_mag = vec![vec![0.0f64; num_bins]; num_frames];
        for f in 0..num_bins {
            let time_slice: Vec<f64> = (0..num_frames).map(|t| mag[t][f]).collect();
            let filtered = filter::median_filter(&time_slice, harmonic_kernel_size);
            for t in 0..num_frames {
                h_mag[t][f] = filtered[t];
            }
        }

        let mut p_mag = vec![vec![0.0f64; num_bins]; num_frames];
        for t in 0..num_frames {
            p_mag[t] = filter::median_filter(&mag[t], percussive_kernel_size);
        }

        let mut harmonic_frames = vec![vec![Complex::new(0.0, 0.0); num_bins]; num_frames];
        let mut percussive_frames = vec![vec![Complex::new(0.0, 0.0); num_bins]; num_frames];

        for t in 0..num_frames {
            for f in 0..num_bins {
                if h_mag[t][f] >= p_mag[t][f] {
                    harmonic_frames[t][f] = self.frames[t][f];
                } else {
                    percussive_frames[t][f] = self.frames[t][f];
                }
            }
        }

        let harmonic = Audio::from_frames(ctx, harmonic_frames, self.data.sample_rate, 1);
        let percussive = Audio::from_frames(ctx, percussive_frames, self.data.sample_rate, 1);
        (harmonic, percussive)
    }
}
