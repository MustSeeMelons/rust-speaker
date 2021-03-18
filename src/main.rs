mod sonar_state;
mod sonar;

use sonar_state::StatusState;

use std::thread;
use std::time::{Duration};

use std::fs::File;
use std::io::BufReader;
use rodio::Source;

// use std::sync::Arc;
// use std::fs;

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

    // Pre-load audio files
    // let welcome_bytes = fs::read("audio/welcome.mp3").expect("Failed to read welcome message!");
    // let w_arc = Arc::new(welcome_bytes);
    // let goodbye_bytes = Arc::new(fs::read("audio/bye.mp3"));

    // Audio setup
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let mut last_state: StatusState = sonar_state.get_state();

    loop {
        sonar_state.state_tick();
        let state: StatusState = sonar_state.get_state();

        match state {
            StatusState::OnTrigger => {
                last_state = state;
            }
            StatusState::OnTriggerEnd => {
                if state != last_state {
                    last_state = state;
                    let welcome = File::open("audio/welcome.mp3").unwrap();
                    let source = rodio::Decoder::new(BufReader::new(welcome)).unwrap();
                    stream_handle.play_raw(source.convert_samples());
                    thread::sleep(Duration::from_millis(1000));
                }
            }
            StatusState::OffTrigger => {
                last_state = state;
            }
            StatusState::OffTriggerEnd => {
                if state != last_state {
                    last_state = state;
                    let goodbye = File::open("audio/bye.mp3").unwrap();
                    let source = rodio::Decoder::new(BufReader::new(goodbye)).unwrap();
                    stream_handle.play_raw(source.convert_samples());
                    thread::sleep(Duration::from_millis(1000));
                }   
            }
        }
    }
}
