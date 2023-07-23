use std::str::FromStr;

use log::Level;

use crate::pages::app::App;
use crate::storage::{get_log_level, is_logging_on};

mod routing;
mod pages;
mod api;
mod storage;
mod ui;
mod hooks;

fn main() {
    if is_logging_on() {
        wasm_logger::init(wasm_logger::Config::new(Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str()).unwrap_or(Level::Warn)));
        log::info!("Logging is turned on");
    }
    log::info!("Starting sheef");
    yew::Renderer::<App>::new().render();

    log::debug!("Append modal container");
    let element = gloo::utils::document().create_element("div").expect("Failed to create div");
    element.set_id("modal-container");
    gloo::utils::body().append_child(&element).expect("Failed to append child");
}