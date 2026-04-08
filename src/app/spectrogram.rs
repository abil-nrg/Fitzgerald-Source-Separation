use num::Complex;

pub const SPECTROGRAM_HEIGHT: usize = 1024;
pub const MAX_TEXTURE_WIDTH: usize = 8192;

pub struct Spectrogram {
    texture: egui::TextureHandle,
}

impl Spectrogram {
    pub fn from_audio(
        ctx: &egui::Context,
        frames: &[Vec<Complex<f64>>],
        window_size: usize,
    ) -> Self {
        let num_bins = window_size / 2 + 1;
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
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        let min_db = max_db - 80.0;
        let db_range = max_db - min_db;

        let w = num_frames.min(MAX_TEXTURE_WIDTH);
        let h = SPECTROGRAM_HEIGHT;
        let mut pixels = vec![egui::Color32::BLACK; w * h];

        for px in 0..w {
            // average over frames in the spectrogram instead
            let frame_start = px * num_frames / w;
            let frame_end = ((px + 1) * num_frames / w)
                .max(frame_start + 1)
                .min(num_frames);
            let count = (frame_end - frame_start) as f64;

            for py in 0..h {
                let bin_f = (1.0 - py as f64 / h as f64) * (num_bins - 1) as f64;
                let bin = bin_f as usize;
                let db_avg = db_matrix[frame_start..frame_end]
                    .iter()
                    .map(|frame| frame[bin.min(num_bins - 1)])
                    .sum::<f64>()
                    / count;
                let normalized = ((db_avg - min_db) / db_range).clamp(0.0, 1.0);
                let c = colorous::INFERNO.eval_continuous(normalized);
                pixels[py * w + px] = egui::Color32::from_rgb(c.r, c.g, c.b);
            }
        }

        let texture = egui::ColorImage::new([w, h], pixels);

        Self {
            texture: ctx.load_texture("spectrogram", texture, egui::TextureOptions::LINEAR),
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        let image = egui::Image::new(&self.texture)
            .shrink_to_fit()
            .maintain_aspect_ratio(false);
        ui.add(image);
    }

}
