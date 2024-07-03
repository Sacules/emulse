use egui_extras::install_image_loaders;
use std::env;

use crate::darkroom::Darkroom;
use crate::lighttable::LightTable;

enum CurrentView {
    LightTable,
    Darkroom,
}

pub struct App {
    current_view: CurrentView,
    light_table: LightTable,
    darkroom: Darkroom,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args: Vec<String> = env::args().collect();
        let filename = match args.len() {
            1 => "".to_string(),
            _ => args[1].clone(),
        };

        install_image_loaders(&cc.egui_ctx);

        Self {
            current_view: CurrentView::LightTable,
            darkroom: Darkroom::new(cc, filename.clone()),
            light_table: LightTable::new(cc),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("nav_bar")
            .exact_height(64.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.heading("Emulse");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let darkroom = ui.add(egui::Button::new(egui::RichText::new("Darkroom")));
                        if darkroom.clicked() {
                            self.current_view = CurrentView::Darkroom;
                        }

                        let lighttable =
                            ui.add(egui::Button::new(egui::RichText::new("Lighttable")));
                        if lighttable.clicked() {
                            self.current_view = CurrentView::LightTable;
                        }
                    });
                });
            });

        match self.current_view {
            CurrentView::Darkroom => self.darkroom.update(ctx, frame),
            CurrentView::LightTable => self.light_table.update(ctx, frame),
        }
    }
}
