#![allow(dead_code)]
use eframe::{
    egui::CentralPanel,
    epi::{App, Frame, Storage},
    run_native, NativeOptions,
};

use chrono::Duration;

use egui::RichText;
use egui_extras::RetainedImage;

mod mpd_ctl;
use mpd_ctl::*;

struct Mpd {
    conn: mpd::Client,
}

enum MusicState {
    Play,
    Stop,
    Pause,
}

impl Mpd {
    fn new() -> Mpd {
        let connection = mpd::Client::connect("127.0.0.1:6600").unwrap();

        Mpd {
            conn: connection,
        }
    }
}

struct Song {
    title: String,
    artist: String,
    alubum: String,
    length: Duration, // あとでchronoを使って時間に書き換える
    cover_img: RetainedImage,
}

impl Song {
    fn new() -> Song {
        let img =
            RetainedImage::from_image_bytes("CHOSYOKU.jpg", include_bytes!("../CHOSYOKU.jpg"))
                .unwrap();
        Self {
            title: "girl my friend".to_string(),
            artist: "マカロニえんぴつ".to_string(),
            alubum: "CHOSYOKU".to_string(),
            cover_img: img,
            length: Duration::seconds(80),
        }
    }
}

impl App for Mpd {
    fn setup(&mut self, ctx: &egui::Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "japanese_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../assets/fonts/SawarabiGothic-Regular.ttf"
            )),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "japanese_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("japanese_font".to_owned());

        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.image(
                    cover(&mut self.conn).texture_id(ctx),
                    egui::Vec2::new(600.0, 600.0),
                );

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(190.0);
                    // ui.style();
                    ui.label(RichText::new(title(&mut self.conn)).size(110.0));
                    ui.label(RichText::new(artist(&mut self.conn)).size(60.0));
                    ui.label(RichText::new(album(&mut self.conn)).size(60.0));
                });
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{}", elapsed_time(&mut self.conn).num_seconds())).size(20.0)); // !todo この処理を関数化する

                ui.add_sized([1170.0, 1.0], egui::ProgressBar::new(progress(&mut self.conn)));

                ui.label(RichText::new(format!("{}", song_length(&mut self.conn).num_seconds())).size(20.0));
                // !todo この処理を関数化する
            });

            ui.put(
                egui::Rect::from_min_size(egui::Pos2::new(1200.0, 3.0), egui::Vec2::new(1.0, 1.0)),
                egui::Image::new(
                    RetainedImage::from_svg_bytes(
                        "play.svg",
                        include_bytes!("../assets/icons/play.svg"),
                    )
                    .unwrap()
                    .texture_id(ctx),
                    egui::Vec2::new(100.0, 100.0),
                ),
            );
        });
    }

    fn name(&self) -> &str {
        "Viewer"
    }
}

fn main() {
    let app = Mpd::new();
    let window_option = NativeOptions::default();
    run_native(Box::new(app), window_option)
}