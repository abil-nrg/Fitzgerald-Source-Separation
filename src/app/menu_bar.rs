use egui::Ui;
use poll_promise::Promise;

const TEST_AUDIO: &[u8] = include_bytes!("../../assets/test.wav");
#[derive(Default)]
pub struct MenuBar {
    file_promise: Option<Promise<Option<Vec<u8>>>>,
}

impl MenuBar {
    fn open_file_dialog(&mut self) {
        let file_promise = poll_promise::Promise::spawn_local(async {
            let file = rfd::AsyncFileDialog::new()
                .add_filter("Audio", &["wav", "mp3", "flac"])
                .pick_file()
                .await;

            if let Some(file) = file {
                let bytes = file.read().await;
                Some(bytes)
            } else {
                None
            }
        });
        self.file_promise = Some(file_promise);
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open Audio").clicked() {
                    self.open_file_dialog();
                }
                if ui.button("Load Example").clicked() {
                    self.file_promise = Some(Promise::from_ready(Some(TEST_AUDIO.to_vec())));
                }
            });
        });
    }

    pub fn poll_file(&mut self) -> Option<Vec<u8>> {
        let promise = self.file_promise.as_ref()?;
        if let Some(result) = promise.ready() {
            let bytes = result.clone();
            self.file_promise = None;
            bytes
        } else {
            None
        }
    }
}
