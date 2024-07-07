use egui::TextureHandle;
use image::DynamicImage;
use mut_rc::MutRc;
use std::{collections::HashMap, rc::Rc};

use crate::app::EmulseState;

#[derive(Debug, Clone)]
pub struct Image {
    data: DynamicImage,
    path: String,
}

pub struct LightTable {
    pub images: Vec<Rc<Image>>,
    pub texture_map: HashMap<String, TextureHandle>,
    state: MutRc<EmulseState>,
}

const TEST_IMAGES: [&str; 12] = [
    "test/roll/01.tif",
    "test/roll/02.tif",
    "test/roll/03.tif",
    "test/roll/04.tif",
    "test/roll/05.tif",
    "test/roll/06.tif",
    "test/roll/07.tif",
    "test/roll/08.tif",
    "test/roll/09.tif",
    "test/roll/10.tif",
    "test/roll/11.tif",
    "test/roll/12.tif",
];

impl LightTable {
    pub fn new(state: MutRc<EmulseState>) -> Self {
        let mut images = Vec::new();

        for img in TEST_IMAGES {
            let data = image::open(img).unwrap();
            let img = Rc::new(Image {
                data,
                path: img.to_string(),
            });
            images.push(img);
        }

        Self {
            images,
            state,
            texture_map: HashMap::new(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .max_width(268.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.collapsing("Library", |ui| {
                        ui.label("random");
                        ui.label("folder");
                        ui.label("johncena");
                    });
                });
            });

        egui::SidePanel::right("right_panel")
            .max_width(300.0)
            .show_animated(ctx, true, |ui| {
                ui.vertical(|ui| {
                    ui.collapsing("Selection", |ui| {
                        ui.label("rotate 90°");
                        ui.label("rotate -90°");
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
                            if i % 5 == 0 {
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
        if !self.texture_map.contains_key(img.path.as_str()) {
            let bytes = img.data.to_rgba8();
            let data = egui::ColorImage::from_rgba_unmultiplied(
                [img.data.width() as usize, img.data.height() as usize],
                &bytes.into_flat_samples().samples,
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
                let _ = self
                    .state
                    .with_mut(|state| state.selected_image_path = img.path.clone());
            }

            f.content_ui
                .label(egui::RichText::new(img.path.as_str()).color(egui::Color32::WHITE));
        }
        f.end(ui);
    }
}
