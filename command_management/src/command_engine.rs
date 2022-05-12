use std::sync::{Arc, Mutex};

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::mp3_player::Mp3Player;

pub struct CommandEngine {
    mp3_player: Arc<Mutex<Mp3Player>>,
    podcast_library: Arc<Mutex<PodcastLibrary>>,
}

impl CommandEngine {
    pub fn new(
        mp3_player: Arc<Mutex<Mp3Player>>,
        podcast_library: Arc<Mutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine {
            mp3_player,
            podcast_library,
        }
    }

    pub fn handle_command(&mut self, _command: &str) {
        let mut mp3_player = self.mp3_player.lock().unwrap();
        mp3_player.pause();
    }

    //pub fn run(this: Arc<Mutex<Self>>) {
    //    println!("Launching thread");
    //    thread::spawn(move || {
    //        println!("Thread launched");
    //        let mut s = String::from("");
    //        while s != "exit" {
    //            print!(">>> ");
    //            // TODO : Use if let here
    //            s = command_reader::read_command().unwrap();
    //            this.lock().unwrap().handle_command(&s);
    //        }
    //        println!("Thread finished");
    //    });
    //}
}
