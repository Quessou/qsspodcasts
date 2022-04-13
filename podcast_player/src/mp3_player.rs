use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

pub struct Mp3Player {

}

impl Mp3Player {
    pub fn play_file(&mut self, path: &str) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}