use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

pub struct Mp3Player {
    sink: Sink,
    stream : OutputStream,
}

impl Mp3Player {
    pub fn new() -> Mp3Player {
        //let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        //let huhu = Sink::try_new(stream_handle);
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Mp3Player { sink, stream }
    }

    pub fn play_file(&mut self, path: &str) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        self.sink.append(source);
        println!("Podcast appended");

        // self.sink.play();

        // TODO : Replace that by something that polls for commands ?
        // self.sink.sleep_until_end();
    }

    pub fn pause(&mut self) {
        println!("Pause !");
        self.sink.pause();
        println!("Pauuuuse !!!");
    }
}
