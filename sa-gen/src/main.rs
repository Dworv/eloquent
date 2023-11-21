use std::{fs::File, io::Read};
use time::Duration;

use metaheuristics::{Metaheuristics, simulated_annealing};
use sim::*;
use rand::prelude::*;
use lazy_static::lazy_static;

lazy_static!{
    static ref TEXT: Vec<Option<Key>> = {
        let mut text = String::new();
        File::open("text/text.txt").unwrap().read_to_string(&mut text).unwrap();
        preprocess_str(&text)
    };
    static ref SPEEDS: Speeds = Speeds::init();
}

struct KeyboardProblem;

impl Metaheuristics<[Key; 30]> for KeyboardProblem {
    fn clone_candidate(&mut self, candidate: &[Key; 30]) -> [Key; 30] {
        candidate.clone()
    }

    fn generate_candidate(&mut self) -> [Key; 30] {
        let mut keys = Key::all();
        keys.shuffle(&mut thread_rng());
        keys
    }

    fn rank_candidate(&mut self, candidate: &[Key; 30]) -> f64 {
        1./sim(&Layout::from_order(*candidate), &SPEEDS, &TEXT)
    }

    fn tweak_candidate(&mut self, candidate: &[Key; 30]) -> [Key; 30] {
        let mut new = candidate.clone();
        let mut rng = thread_rng();
        let start = rng.gen_range(0..30);
        let mut end = rng.gen_range(0..29);

        if end >= start {
            end += 1;
        }

        new.swap(start, end);
        new
    }
}

fn main() {
    let solution = simulated_annealing::solve(&mut KeyboardProblem, Duration::minutes(10));
    println!("time: {:.1} solution {:?}", sim(&Layout::from_order(solution), &SPEEDS, &TEXT), solution)
}
