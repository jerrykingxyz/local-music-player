use rodio::OutputStream;
use std::sync::Arc;
mod error;
mod file;
mod player_stream;

//use file_source::FileSource;
use player_stream::Controller;

pub struct Player {
    controller: Arc<Controller>, //    second: i32,
                                 //    status: PlayerStatus,
}

impl Player {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (controller, output) = player_stream::new_stream();
        stream_handle.play_raw(output);
        Self { controller }
    }
    pub fn play(&mut self, file_path: String) {
        //        self.controller.lock();
        //        controller.file = Some(file::File::new(file_path)?);
        //        Ok(())
    }
    pub fn pause(&mut self) {}
    pub fn set_volume(&mut self) {}
}
