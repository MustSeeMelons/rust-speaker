use rppal::gpio::{Gpio, OutputPin};
use crate::sonar::Sonar;

const GPIO_LED: u8 = 23;
// Turn on distance
const TRIGGER_DISTANCE: f64 = 40.0;
// Turn off distance
const TRIGGER_END_DISTANCE: f64 = TRIGGER_DISTANCE * 2.0;
const DIST_SAMPLE_COUNT: isize = 5;

#[derive(Debug)]
pub enum StatusState {
    OnTrigger,
    OnTriggerEnd,
    OffTrigger,
    OffTriggerEnd,
}

pub struct SonarState {
    sonar: Sonar,
    led: OutputPin,
    status_state: StatusState,
}

impl SonarState {
    pub fn new() -> Option<SonarState> {
        let sonar = match Sonar::new() {
            Some(sonar) => sonar,
            None => {
                println!("Failed to create Sonar");
                return None;
            }
        };

        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };

        let mut status_led = match gpio.get(GPIO_LED) {
            Ok(pin) => pin.into_output(),
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };

        status_led.set_low();

        Some(SonarState { sonar, led: status_led, status_state: StatusState::OnTrigger })
    }

    fn toggle_led(&mut self) {
        self.led.toggle();
    }

    fn vector_median(&self, vector: &Vec<f64>) -> f64 {
        if vector.len() % 2 != 0 {
            vector[vector.len() / 2]
        } else {
            let lower = vector[(vector.len() as f64 / 2.0).floor() as usize];
            let upper = vector[(vector.len() as f64 / 2.0).ceil() as usize];

            (lower + upper) / 2.0
        }
    }

    /**
        Filtering our sample set, returning the median value.
    */
    fn filter(&mut self, samples: &mut Vec<f64>) -> f64 {
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = self.vector_median(&samples);

        median
    }


    pub fn state_tick(&mut self) {
        // Mutable, because we need to call functions on it
        let mut samples: Vec<f64> = Vec::new();

        for _ in 0..DIST_SAMPLE_COUNT {
            let dist = self.sonar.get_distance();
            if dist > 0.0 {
                samples.push(dist);
            }
        }

        // Passing as a mutable reference to the function can change the variable
        let distance = self.filter(&mut samples);

        // println!("{:?}", samples);
        println!("d: {}, s: {:?}", distance, self.status_state);

        match self.status_state {
            StatusState::OnTrigger => {
                if distance < TRIGGER_DISTANCE {
                    self.status_state = StatusState::OnTriggerEnd;
                    self.toggle_led();
                }
            }
            StatusState::OnTriggerEnd => {
                if distance > TRIGGER_END_DISTANCE {
                    self.status_state = StatusState::OffTrigger;
                }
            }
            StatusState::OffTrigger => {
                if distance < TRIGGER_DISTANCE {
                    self.status_state = StatusState::OffTriggerEnd;
                    self.toggle_led();
                }
            }
            StatusState::OffTriggerEnd => {
                if distance > TRIGGER_END_DISTANCE {
                    self.status_state = StatusState::OnTrigger;
                }
            }
        }
    }

    pub fn get_state(&mut self) -> &StatusState {
        &self.status_state
    }
}