pub struct LightTable {}

impl LightTable {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for LightTable {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
    }
}
