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

impl Metaheuristics<[Key; 20]> for KeyboardProblem {
    fn clone_candidate(&mut self, candidate: &[Key; 20]) -> [Key; 20] {
        candidate.clone()
    }

    fn generate_candidate(&mut self) -> [Key; 20] {
        let mut keys = Key::all20();
        keys.shuffle(&mut thread_rng());
        keys
    }

    fn rank_candidate(&mut self, candidate: &[Key; 20]) -> f64 {
        1./sim(&Layout::from_20(*candidate), &SPEEDS, &TEXT)
    }

    fn tweak_candidate(&mut self, candidate: &[Key; 20]) -> [Key; 20] {
        let mut new = candidate.clone();
        let mut rng = thread_rng();
        let start = rng.gen_range(0..20);
        let mut end = rng.gen_range(0..19);

        if end >= start {
            end += 1;
        }

        new.swap(start, end);
        new
    }
}

fn main() {
    let solution = simulated_annealing::solve(&mut KeyboardProblem, Duration::seconds(3));
    println!("time: {:.1} solution {:?}", sim(&Layout::from_20(solution), &SPEEDS, &TEXT), &Layout::from_20(solution).to_keys())
}
