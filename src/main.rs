#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::app::App;
use miniquad as mq;

pub mod app;
//pub mod darkroom;
pub mod lighttable;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let conf = mq::conf::Conf {
        high_dpi: true,
        ..Default::default()
    };

    mq::start(conf, || Box::new(App::new()));
}
