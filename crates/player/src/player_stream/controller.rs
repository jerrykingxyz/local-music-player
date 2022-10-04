use crate::file::File;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct Controller {
    pub volume: Mutex<f32>,
    pub speed: Mutex<f32>,
    pub pause: AtomicBool,
    pub file: Option<File>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            volume: Mutex::new(1.0),
            speed: Mutex::new(1.0),
            pause: AtomicBool::new(false),
            file: None,
        }
    }
}
