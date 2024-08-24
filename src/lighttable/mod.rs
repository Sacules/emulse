pub mod db;
pub mod image;

use egui::TextureHandle;
use mut_rc::MutRc;
use std::collections::HashMap;
use std::sync::Arc;

use crate::app::{CurrentView, EmulseState};
use crate::lighttable::image::Image;

pub struct LightTable {
    pub images: Vec<Arc<Image>>,
    pub texture_map: HashMap<String, TextureHandle>,

    state: MutRc<EmulseState>,
}

impl LightTable {
    pub fn new(state: MutRc<EmulseState>) -> Self {
        Self {
            images: vec![],
            state,
            texture_map: HashMap::new(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);

                    egui::CollapsingHeader::new("Library").show_unindented(ui, |ui| {
                        ui.add_space(8.0);
                        ui.collapsing("test", |ui| {
                            ui.label("roll");
                        });
                        ui.add_space(8.0);
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let mut grid_builder = egui_grid::GridBuilder::new().spacing(16.0, 16.0);
                        for i in 0..self.images.len() {
                            if i % 4 == 0 {
                                grid_builder =
                                    grid_builder.new_row(egui_extras::Size::exact(250.0));
                            }

                            grid_builder = grid_builder.cell(egui_extras::Size::exact(250.0));
                        }

                        // Only show after preallocating enough space
                        grid_builder.show(ui, |mut grid| {
                            for img in self.images.clone() {
                                grid.cell(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        self.image_slide(ctx, ui, &img);
                                    });
                                });
                            }
                        });
                    });
            });
        });
    }

    fn image_slide(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, img: &Image) {
        //TODO: move this to another function, only leave ui stuff here
        if !self.texture_map.contains_key(img.path.as_str()) {
            // TODO: dynamically infer this, as some files can be in 16 bit
            let bytes = img.data.as_rgb8().unwrap();
            let data = egui::ColorImage::from_rgb(
                [img.data.width() as usize, img.data.height() as usize],
                bytes,
            );
            let handle = ctx.load_texture(img.path.clone(), data, Default::default());
            self.texture_map.insert(img.path.to_string(), handle);
        }

        let handle = self.texture_map.get(img.path.as_str()).unwrap();

        let mut f = egui::Frame::default().inner_margin(32.0).begin(ui);
        f.frame.fill = egui::Color32::DARK_GRAY;
        {
            let image = egui::Image::new(handle)
                .show_loading_spinner(true)
                .maintain_aspect_ratio(true)
                .sense(egui::Sense {
                    click: true,
                    drag: false,
                    focusable: true,
                })
                .fit_to_exact_size((176.0, 176.0).into());
            let resp = f.content_ui.add(image);

            if resp.hovered() {
                f.frame.fill = egui::Color32::GRAY;
            }

            if resp.double_clicked() {
                let _ = self.state.with_mut(|state| {
                    state.selected_image_path = img.path.clone();
                    state.current_view = CurrentView::Darkroom;
                });
            }

            f.content_ui
                .label(egui::RichText::new(img.path.as_str()).color(egui::Color32::WHITE));
        }
        f.end(ui);
    }
}
