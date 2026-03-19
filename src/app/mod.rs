use symphonia::core::io::MediaSourceStream;
use fitzgerald_source_separation::audio::AudioData;

pub mod menu_bar;

#[derive(Default)]
pub struct SeparationApp {
    menu_bar: menu_bar::MenuBar,
    loaded_audio: Option<AudioData>,
    current_stream: Option<cpal::Stream>
}

impl SeparationApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            menu_bar: menu_bar::MenuBar::default(),
            loaded_audio: None,
            current_stream: None,
        }
    }

    fn ui_waveform(&self, ui: &mut egui::Ui, audio: &AudioData) {
        let desired_size = egui::vec2(ui.available_width(), 120.0);
        let (rect, _response) = ui.allocate_at_least(desired_size, egui::Sense::hover());

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 2.0, ui.visuals().extreme_bg_color);

        let mono = audio.to_mono();
        let middle_y = rect.center().y;
        let width = rect.width();
        let height = rect.height();

        let step = (mono.len() as f32 / width).max(1.0) as usize;
        let points: Vec<egui::Pos2> = mono
            .iter()
            .step_by(step)
            .enumerate()
            .map(|(x, &sample)| {
                let x_pos = rect.left() + x as f32;
                let y_offset = sample * (height / 2.0);
                egui::pos2(x_pos, middle_y - y_offset)
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
                    log::info!("loaded audio: {} samples", data.samples.len());
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
                if ui.button("Play original").clicked(){
                    match fitzgerald_source_separation::audio::play_audio(audio){
                        Ok(stream) => self.current_stream = Some(stream),
                        Err(e) => log::error!("playback failed: {}", e),
                    }
                }

                if ui.button("Stop").clicked(){
                    self.current_stream = None;
                }

                ui.label(format!("Sample rate: {} Hz", audio.sample_rate));

                self.ui_waveform(ui,audio);
            } else {
                ui.label("load an audio file.");
            }
        });
    }
}
