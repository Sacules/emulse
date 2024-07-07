use mut_rc::MutRc;
use {egui_miniquad as egui_mq, miniquad as mq};

use crate::darkroom::Darkroom;
use crate::lighttable::LightTable;

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
    darkroom: Option<Darkroom>,

    state: MutRc<EmulseState>,
}

impl App {
    pub fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();
        let egui_mq = egui_mq::EguiMq::new(&mut *mq_ctx);

        let state = MutRc::new(EmulseState::default());

        let light_table = LightTable::new(state.clone());
        let darkroom = None;
        let current_view = CurrentView::LightTable;

        Self {
            egui_mq,
            mq_ctx,
            current_view,
            darkroom,
            light_table,
            state,
        }
    }
}

impl mq::EventHandler for App {
    fn update(&mut self) {
        match self.current_view {
            CurrentView::Darkroom => match &self.darkroom {
                Some(_) => {}
                None => {
                    let handle = self
                        .light_table
                        .texture_map
                        .get(&self.state.get_clone().unwrap().selected_image_path)
                        .unwrap();

                    self.darkroom = Some(Darkroom::new(self.mq_ctx.as_mut(), handle.to_owned()));
                }
            },
            CurrentView::LightTable => {}
        }
    }

    fn draw(&mut self) {
        self.egui_mq.run(self.mq_ctx.as_mut(), |_, ctx| {
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
                CurrentView::Darkroom => match &self.darkroom {
                    Some(_d) => self.darkroom.as_mut().unwrap().ui(ctx),
                    None => {}
                },
                CurrentView::LightTable => self.light_table.ui(ctx),
            }
        });

        self.egui_mq.draw(&mut *self.mq_ctx);

        match self.current_view {
            CurrentView::Darkroom => match &self.darkroom {
                Some(darkroom) => darkroom.update(&mut *self.mq_ctx),
                None => {}
            },
            CurrentView::LightTable => {}
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
