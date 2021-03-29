mod sonar;
mod sonar_state;
use sonar_state::StatusState;
use std::thread;

use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Sonar setup
    let mut sonar_state = match sonar_state::SonarState::new() {
        Some(state) => state,
        None => {
            println!("Failed to create state");
            return;
        }
    };

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
                    // This works fine, except can't be used to play a song, as I wont be able to stop it
                    thread::spawn(|| {
                        play::play("audio/welcome.mp3").unwrap();
                    });

                    // let file = File::open("audio/welcome.mp3").unwrap();
                    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    // stream_handle.play_raw(source.convert_samples()).unwrap();
                }
            }
            StatusState::OffTrigger => {
                last_state = state;
            }
            StatusState::OffTriggerEnd => {
                if state != last_state {
                    last_state = state;

                    thread::spawn(|| {
                        play::play("audio/bye.mp3").unwrap();
                    });

                    // let file = File::open("audio/bye.mp3").unwrap();
                    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    // stream_handle.play_raw(source.convert_samples()).unwrap();
                }
            }
        }
    }
}
