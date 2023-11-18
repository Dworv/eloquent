use std::{fs::File, f64::INFINITY};

use serde::Deserialize;

use crate::{finger, row};

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
        let file = File::open("speeds.json")
            .unwrap_or(File::open("analysis/speeds.json").unwrap_or(File::open("../analysis/speeds.json").unwrap()));
        let mut speeds: Speeds = serde_json::from_reader(file).unwrap();
        speeds.set_min();
        speeds
    }

    fn set_min(&mut self) {
        for time in self.finger {
            if time < self.min {
                self.min = time;
            }
        }

        for time in self.row.iter().flatten() {
            if *time < self.min {
                self.min = *time;
            }
        }
    }

    pub fn time(&self, start: u8, end: u8) -> f64 {
        if start == end {
            self.min_time()
        } else {
            (self.finger[finger(start) as usize]
            + self.row[row(start) as usize][row(end) as usize])
            / 2.0
        }
    }

    pub fn min_time(&self) -> f64 {
        self.min / 2.
    }

    #[allow(unused)]
    pub(crate) fn test_new(finger: [f64; 8], row: [[f64; 3]; 3]) -> Speeds {
        let mut speeds = Speeds {
            finger,
            row,
            min: inf(),
        };
        speeds.set_min();
        speeds
    }
}
