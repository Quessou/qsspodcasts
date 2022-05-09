use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

pub struct Mp3Player {
    sink: Option<Sink>,
}

impl Mp3Player {
    pub fn new() -> Mp3Player {
        Mp3Player { sink: Option::None }
    }

    pub fn play_file(&mut self, path: &str) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        self.sink = Option::Some(Sink::try_new(&stream_handle).unwrap());
        self.sink.as_ref().unwrap().append(source);

        // TODO : Replace that by something that polls for commands ?
        self.sink.as_ref().unwrap().sleep_until_end();
    }

    pub fn pause(&mut self) {
        println!("Pause !");
        self.sink.as_ref().unwrap().pause();
        println!("Pauuuuse !!!");
    }
}
