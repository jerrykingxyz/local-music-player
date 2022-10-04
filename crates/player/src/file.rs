use rodio::Decoder;
use std::fs::File as FsFile;
use std::io::BufReader;

use super::error::Error;

pub type FileSource = Decoder<BufReader<FsFile>>;

pub struct File {
    file_path: String,
    pub source: FileSource,
}

impl File {
    pub fn new(file_path: String) -> Result<Self, Error> {
        let file = BufReader::new(FsFile::open(file_path.to_string())?);
        Ok(Self {
            file_path,
            source: Decoder::new(file).unwrap(),
        })
    }
}
