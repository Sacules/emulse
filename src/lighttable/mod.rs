use egui::{TextureHandle, TextureId};
use image::DynamicImage;
use std::collections::HashMap;

pub struct LightTable {
    pub images: Vec<DynamicImage>,
    pub texture_map: HashMap<String, TextureHandle>,
}

const TEST_IMAGES: [&str; 12] = [
    "test/roll/01.jpg",
    "test/roll/02.jpg",
    "test/roll/03.jpg",
    "test/roll/04.jpg",
    "test/roll/05.jpg",
    "test/roll/06.jpg",
    "test/roll/07.jpg",
    "test/roll/08.jpg",
    "test/roll/09.jpg",
    "test/roll/10.jpg",
    "test/roll/11.jpg",
    "test/roll/12.jpg",
];

impl LightTable {
    pub fn new() -> Self {
        let mut images = Vec::new();

        for img in TEST_IMAGES {
            let data = image::open(img).unwrap();
            images.push(data);
        }

        Self {
            images,
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
                        for i in 0..TEST_IMAGES.len() {
                            if i % 5 == 0 {
                                grid_builder =
                                    grid_builder.new_row(egui_extras::Size::exact(250.0));
                            }

                            grid_builder = grid_builder.cell(egui_extras::Size::exact(250.0));
                        }

                        // Only show after preallocating enough space
                        grid_builder.show(ui, |mut grid| {
                            for (i, img) in self.images.clone().into_iter().enumerate() {
                                grid.cell(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        self.image_slide(ctx, ui, TEST_IMAGES[i], &img);
                                    });
                                });
                            }
                        });
                    });
            });
        });
    }

    fn image_slide(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        path: &str,
        img: &DynamicImage,
    ) {
        if !self.texture_map.contains_key(&path.to_string()) {
            let bytes = img.to_rgba8();
            let data = egui::ColorImage::from_rgba_unmultiplied(
                [img.width() as usize, img.height() as usize],
                &bytes.into_flat_samples().samples,
            );

            let handle = ctx.load_texture(path, data, Default::default());
            self.texture_map.insert(path.to_string(), handle);
        }

        let handle = self.texture_map.get(&path.to_string()).unwrap();

        let mut f = egui::Frame::default().inner_margin(32.0).begin(ui);
        f.frame.fill = egui::Color32::DARK_GRAY;
        {
            let image = egui::Image::new(handle)
                .show_loading_spinner(true)
                .maintain_aspect_ratio(true)
                .fit_to_exact_size((176.0, 176.0).into());
            let resp = f.content_ui.add(image);

            if resp.hovered() {
                f.frame.fill = egui::Color32::GRAY;
            }

            f.content_ui
                .label(egui::RichText::new(path).color(egui::Color32::WHITE));
        }
        f.end(ui);
    }
}
