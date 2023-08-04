/// TODO: Remove me
use log::{error, warn};
use std::sync::{Arc, Mutex, MutexGuard};

use path_providing::path_provider::PathProvider;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

use crate::mp3_player::Mp3Player;
use crate::player_error::PlayerError;

use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct SymphoniaMp3Player {
    selected_episode: Option<PodcastEpisode>,
    path_provider: Arc<Mutex<Box<dyn PathProvider>>>,
}

impl SymphoniaMp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> SymphoniaMp3Player {
        SymphoniaMp3Player {
            selected_episode: None,
            path_provider: Arc::new(Mutex::new(path_provider)),
        }
    }
}

impl Mp3Player for SymphoniaMp3Player {
    fn get_path_provider(&self) -> MutexGuard<Box<dyn PathProvider>> {
        self.path_provider.lock().unwrap()
    }
    fn get_selected_episode(&self) -> &Option<PodcastEpisode> {
        &self.selected_episode
    }
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
        self.selected_episode = episode;
    }

    fn is_paused(&self) -> bool {
        false
    }

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError> {
        // Open the media source.
        let src = std::fs::File::open(&path).expect("failed to open media");

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Create a probe hint using the file's extension. [Optional]
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        // Get the instantiated format reader.
        let mut format = probed.format;

        // Find the first audio track with a known (decodeable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks");

        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        // Store the track identifier, it will be used to filter packets.
        let track_id = track.id;

        // The decode loop.
        loop {
            // Get the next packet from the media format.
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and it is not
                    // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                    // for chained OGG physical streams.
                    unimplemented!();
                }
                Err(err) => {
                    // A unrecoverable error occured, halt decoding.
                    panic!("{}", err);
                }
            };

            // Consume any new metadata that has been read since the last packet.
            while !format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // Consume the decoded audio samples (see below).
                    // TODO : https://github.com/pdeljanov/Symphonia/blob/master/GETTING_STARTED.md
                }
                Err(Error::IoError(_)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    continue;
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    continue;
                }
                Err(err) => {
                    // An unrecoverable error occured, halt decoding.
                    panic!("{}", err);
                }
            }
        }
        Ok(())
    }

    fn pause(&mut self) {}
    fn play(&mut self) {}
}

unsafe impl Send for SymphoniaMp3Player {}
