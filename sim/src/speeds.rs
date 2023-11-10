use std::{fs::File, f64::INFINITY};

use serde::Deserialize;

use crate::Slot;

fn inf() -> f64 {
    INFINITY
}

#[derive(Deserialize, Debug)]
pub struct Speeds {
    finger: [f64; 8],
    row: [[f64; 3]; 3],
    #[serde(default = "inf")]
    min: f64,
}

impl Speeds {
    pub fn init() -> Speeds {
        let file = File::open("speeds.json").unwrap_or(File::open("analysis/speeds.json").unwrap());
        let mut speeds: Speeds = serde_json::from_reader(file).unwrap();

        for time in &speeds.finger {
            if time < &speeds.min {
                speeds.min = *time;
            }
        }

        for time in speeds.row.iter().flatten() {
            if time < &speeds.min {
                speeds.min = *time;
            }
        }

        speeds
    }

    pub fn time(&self, start: Slot, end: Slot) -> f64 {
        if start == end {
            self.min
        } else {
            (self.finger[start.finger().index() as usize]
            + self.row[start.row() as usize][end.row() as usize])
            / 2.0
        }
    }

    pub fn space_time(&self) -> f64 {
        self.min / 2.
    }
}
