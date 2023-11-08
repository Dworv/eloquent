use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Speeds {
    finger: [f64; 8],
    row: [[f64; 3]; 3],
}

impl Speeds {
    pub fn init() -> Speeds {
        let file = File::open("speeds.json").unwrap_or(File::open("analysis/speeds.json").unwrap());
        serde_json::from_reader(file).unwrap()
    }

    pub fn time() {}
}
