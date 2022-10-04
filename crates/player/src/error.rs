use std::convert::From;
//use rodio::;

pub struct Error {
    message: String,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            message: "open file failed".to_string(),
        }
    }
}
