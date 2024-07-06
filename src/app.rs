use mut_rc::MutRc;
use std::env;

use {egui_miniquad as egui_mq, miniquad as mq};

use crate::darkroom::Darkroom;
use crate::lighttable::LightTable;

#[derive(PartialEq, Debug, Clone)]
enum CurrentView {
    LightTable,
    Darkroom,
}

#[derive(Debug, Clone, Default)]
pub struct EmulseState {
    pub selected_image_path: String,
}

pub struct App {
    /// Bindings to egui for miniquad
    egui_mq: egui_mq::EguiMq,

    /// Renderer
    mq_ctx: Box<dyn mq::RenderingBackend>,

    current_view: CurrentView,
    light_table: LightTable,
    darkroom: Darkroom,

    state: MutRc<EmulseState>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let _filename = match args.len() {
            1 => "".into(),
            _ => args[1].clone(),
        };

        let mut mq_ctx = mq::window::new_rendering_backend();
        let egui_mq = egui_mq::EguiMq::new(&mut *mq_ctx);

        let state = MutRc::new(EmulseState::default());
        let light_table = LightTable::new(state.clone());

        Self {
            egui_mq,
            mq_ctx,
            current_view: CurrentView::LightTable,
            darkroom: Darkroom::new(),
            light_table,
            state,
        }
    }
}

impl mq::EventHandler for App {
    fn update(&mut self) {
        match self.current_view {
            CurrentView::Darkroom => {
                if !self.darkroom.ready {
                    let handle = self
                        .light_table
                        .texture_map
                        .get(&self.state.get_clone().unwrap().selected_image_path)
                        .unwrap();
                    self.darkroom.prepare(&mut *self.mq_ctx, handle.id());
                }
            }
            CurrentView::LightTable => {}
        }
    }

    fn draw(&mut self) {
        self.egui_mq.run(&mut *self.mq_ctx, |_, ctx| {
            egui_extras::install_image_loaders(ctx);

            egui::TopBottomPanel::top("nav_bar")
                .exact_height(64.0)
                .show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.heading("Emulse");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let darkroom =
                                ui.add(egui::Button::new(egui::RichText::new("Darkroom")));
                            if darkroom.clicked()
                                && self.state.get_clone().unwrap().selected_image_path
                                    != String::new()
                            {
                                //TODO: retrieve filename of current image
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
                CurrentView::Darkroom => self.darkroom.ui(ctx),
                CurrentView::LightTable => self.light_table.ui(ctx),
            }
        });

        self.egui_mq.draw(&mut *self.mq_ctx);

        if self.current_view == CurrentView::Darkroom && self.darkroom.ready {
            self.darkroom.update(&mut *self.mq_ctx);
        }

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}
