mod controller;
mod output;

pub use controller::Controller;
use output::Output;
use std::sync::Arc;

pub fn new_stream() -> (Arc<Controller>, Output) {
    let ctrl = Arc::new(controller::Controller::new());
    let output = output::Output::new(ctrl.clone());
    (ctrl, output)
}
