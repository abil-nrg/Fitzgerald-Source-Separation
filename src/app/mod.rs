use std::io::Cursor;

use symphonia::core::io::MediaSourceStream;

pub mod menu_bar;

#[derive(Default)]
pub struct SeparationApp {
    menu_bar: menu_bar::MenuBar,
}

impl SeparationApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for SeparationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(f) = self.menu_bar.poll_file() {
            let mss = MediaSourceStream::new(Box::new(Cursor::new(f)), Default::default());
            let probe = symphonia::default::get_probe().format(
                &Default::default(),
                mss,
                &Default::default(),
                &Default::default(),
            );
            match probe {
                Ok(probe) => {
                    let format = probe.format;
                    let track = format
                        .tracks()
                        .iter()
                        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL);
                    if let Some(track) = track {
                        log::info!("probed file with codec: {:?}", track.codec_params.codec);
                    } else {
                        log::warn!("probed file, but no valid audio track found");
                    }
                }
                Err(e) => {
                    log::error!("error probing file: {e}");
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar.draw(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Source Separation");
        });
    }
}
