use std::env;

use crate::darkroom::Darkroom;

enum CurrentView {
    LightTable,
    Darkroom,
}

pub struct App {
    current_view: CurrentView,
    darkroom: Darkroom,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args: Vec<String> = env::args().collect();
        let filename = match args.len() {
            0 => "".to_string(),
            _ => args[1].clone(),
        };

        Self {
            current_view: CurrentView::Darkroom,
            darkroom: Darkroom::new(cc, filename.clone()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.current_view {
            CurrentView::Darkroom => self.darkroom.update(ctx, frame),
            CurrentView::LightTable => {} //CurrentView::LightTable => self.light_table.update(ctx, frame),
        }
    }
}
