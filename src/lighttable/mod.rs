pub mod db;
pub mod image;

use ::image::DynamicImage;
use db::Database;
use egui::TextureHandle;
use mut_rc::MutRc;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::app::{CurrentView, EmulseState};
use crate::lighttable::image::Image;

pub struct LightTable {
    pub images: Vec<Arc<Image>>,
    pub texture_map: HashMap<PathBuf, TextureHandle>,

    state: MutRc<EmulseState>,
    db: Database,
    db_images: Vec<db::Image>,
}

impl LightTable {
    pub fn new(state: MutRc<EmulseState>) -> Self {
        let db = Database::new();
        let db_images = db.get_images().unwrap();

        Self {
            db,
            db_images,
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

                        if ui.button("Import").clicked() {
                            let folder = rfd::FileDialog::new()
                                .add_filter("image", &["BMP", "tif", "jpg"])
                                .set_directory(".")
                                .pick_folder()
                                .unwrap();

                            let mut images = Vec::new();
                            for file in folder.read_dir().unwrap() {
                                let path = file.unwrap().path().to_string_lossy().to_string();
                                images.push(db::Image {
                                    path,
                                    ..Default::default()
                                });
                            }

                            let _ = self.db.insert_images(&images);
                            self.db_images.append(&mut images);

                            let res = image::load_from_dir(folder).unwrap();
                            self.images = res;
                        }
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
        if !self.texture_map.contains_key(&img.path) {
            let size = [img.data.width() as usize, img.data.height() as usize];
            let data = match &img.data {
                DynamicImage::ImageRgb8(_) => {
                    let bytes = img.data.as_rgb8().unwrap().as_flat_samples();
                    egui::ColorImage::from_rgb(size, bytes.as_slice())
                }
                DynamicImage::ImageRgb16(_) | DynamicImage::ImageLuma16(_) => {
                    let bytes = img.data.to_rgb8();
                    egui::ColorImage::from_rgb(size, bytes.as_flat_samples().as_slice())
                }
                _ => todo!(),
            };
            let handle = ctx.load_texture(img.path.to_str().unwrap(), data, Default::default());
            self.texture_map.insert(img.path.clone(), handle);
        }

        let handle = self.texture_map.get(&img.path).unwrap();

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
                    state.selected_image_path = Some(img.path.clone());
                    state.current_view = CurrentView::Darkroom;
                });
            }

            f.content_ui
                .label(egui::RichText::new(img.path.to_str().unwrap()).color(egui::Color32::WHITE));
        }
        f.end(ui);
    }
}
