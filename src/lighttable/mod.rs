pub struct LightTable {}

impl LightTable {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for LightTable {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            "file://test/roll/13.jpg",
            "file://test/roll/14.jpg",
            "file://test/roll/15.jpg",
            "file://test/roll/16.jpg",
            "file://test/roll/17.jpg",
            "file://test/roll/18.jpg",
            "file://test/roll/19.jpg",
            "file://test/roll/20.jpg",
            "file://test/roll/21.jpg",
            "file://test/roll/22.jpg",
            "file://test/roll/23.jpg",
            "file://test/roll/24.jpg",
            "file://test/roll/25.jpg",
            "file://test/roll/26.jpg",
            "file://test/roll/27.jpg",
            "file://test/roll/28.jpg",
            "file://test/roll/29.jpg",
            "file://test/roll/30.jpg",
            "file://test/roll/31.jpg",
            "file://test/roll/32.jpg",
            "file://test/roll/33.jpg",
            "file://test/roll/34.jpg",
            "file://test/roll/35.jpg",
            "file://test/roll/36.jpg",
        ];

        egui::SidePanel::left("left_panel")
            .min_width(268.0)
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
            .min_width(300.0)
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
