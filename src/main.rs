mod sonar_state;
mod sonar;
use sonar_state::StatusState;

use std::thread;
use std::time::{Duration};
// use play::*;

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
                    play::play("audio/welcome.mp3").unwrap();
                }
            }
            StatusState::OffTrigger => {
                last_state = state;
            }
            StatusState::OffTriggerEnd => {
                if state != last_state {
                    last_state = state;
                    play::play("audio/bye.mp3").unwrap();
                }   
            }
        }
    }
}
