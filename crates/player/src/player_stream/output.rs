use super::controller::Controller;
use crate::file::FileSource;
use rodio::Source;
use std::sync::Arc;
use std::time::Duration;

pub struct Output {
    controller: Arc<Controller>,
}

impl Output {
    pub fn new(controller: Arc<Controller>) -> Self {
        Self { controller }
    }
    pub fn get_source(&self) -> Option<&FileSource> {
        self.controller.file.as_ref().map(|file| &file.source)
    }
}

impl Source for Output {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.get_source()
            .and_then(|source| source.current_frame_len())
    }

    #[inline]
    fn channels(&self) -> u16 {
        match self.get_source() {
            Some(source) => source.channels(),
            None => 0,
        }
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        match self.get_source() {
            Some(source) => source.sample_rate(),
            None => 0,
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for Output {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
