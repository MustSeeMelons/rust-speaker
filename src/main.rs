mod sonar_state;
mod sonar;

use sonar_state::StatusState;

use std::thread;
use std::time::{Duration};

use std::fs::File;
use std::io::BufReader;
use rodio::Source;

fn main() {
    // Sonar setup
    let mut sonar_state = match sonar_state::SonarState::new()
    {
        Some(state) => state,
        None => {
            println!("Failed to create state");
            return;
        }
    };

    // Audio setup
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let welcome: &'static File = &File::open("audio/welcome.mp3").unwrap();
    let goodbye: &'static File = &File::open("audio/bye.mp3").unwrap();

    loop {
        sonar_state.state_tick();
        let state = sonar_state.get_state();

        match state {
            StatusState::OnTrigger => {
                // TODO make this a function where we pass in the file
                let source = rodio::Decoder::new(BufReader::new(&welcome)).unwrap();
                stream_handle.play_raw(source.convert_samples());
                thread::sleep(Duration::from_millis(1000));
            }
            StatusState::OnTriggerEnd => {
                // Nothing
            }
            StatusState::OffTrigger => {
                let source = rodio::Decoder::new(BufReader::new(&goodbye)).unwrap();
                stream_handle.play_raw(source.convert_samples());
                thread::sleep(Duration::from_millis(1000));
            }
            StatusState::OffTriggerEnd => {
                // Nothing
            }
        }
    }
}
