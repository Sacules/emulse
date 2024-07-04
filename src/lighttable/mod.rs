pub struct LightTable {}

impl LightTable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let test_images = [
            "file://test/roll/01.jpg",
            "file://test/roll/02.jpg",
            "file://test/roll/03.jpg",
            "file://test/roll/04.jpg",
            "file://test/roll/05.jpg",
            "file://test/roll/06.jpg",
            "file://test/roll/07.jpg",
            "file://test/roll/08.jpg",
            "file://test/roll/09.jpg",
            "file://test/roll/10.jpg",
            "file://test/roll/11.jpg",
            "file://test/roll/12.jpg",
        ];

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
                        for i in 0..test_images.len() {
                            if i % 5 == 0 {
                                grid_builder =
                                    grid_builder.new_row(egui_extras::Size::exact(250.0));
                            }

                            grid_builder = grid_builder.cell(egui_extras::Size::exact(250.0));
                        }

                        // Only show after preallocating enough space
                        grid_builder.show(ui, |mut grid| {
                            for img in test_images {
                                grid.cell(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        image_slide(ui, img);
                                    });
                                });
                            }
                        });
                    });
            });
        });
    }
}

fn image_slide(ui: &mut egui::Ui, img: &str) {
    let mut f = egui::Frame::default().inner_margin(32.0).begin(ui);
    f.frame.fill = egui::Color32::DARK_GRAY;
    {
        let resp = f.content_ui.add(
            egui::Image::new(img)
                .maintain_aspect_ratio(true)
                .fit_to_exact_size((176.0, 176.0).into()),
        );

        if resp.hovered() {
            f.frame.fill = egui::Color32::GRAY;
        }

        f.content_ui
            .label(egui::RichText::new(img).color(egui::Color32::WHITE));
    }
    f.end(ui);
}
