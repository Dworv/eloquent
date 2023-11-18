use std::{fs::File, io::Read, f64::INFINITY};

use rand::prelude::*;
use rayon::prelude::*;
use sim::{Key, Layout, sim, Speeds, preprocess_str};

fn main() {
    let mut rng = thread_rng();
    let mut candidates: Vec<[Key; 30]> = Vec::new();
    let speeds = Speeds::init();
    let mut text = String::new();
    File::open("text/text.txt").unwrap().read_to_string(&mut text).unwrap();
    let text = preprocess_str(&text);

    for _ in 0..10 {
        let mut keys = Key::all();
        keys.shuffle(&mut rng);
        candidates.push(keys);
    }

    for i in 1..=200 {
        for og in 0..10 {
            for _ in 0..10 {
                let old = &candidates[og];
                let mut new = old.clone();
                for _ in 0..3 {
                    let start = rng.gen_range(0..30);
                    let mut end = rng.gen_range(0..29);

                    if end >= start {
                        end += 1;
                    }

                    new.swap(start, end);
                }
                candidates.push(new);
            }
        }
        candidates = filter_candidates(candidates, &speeds, &text);
        println!("finished batch #{}", i);
    }
}

fn filter_candidates(candidates: Vec<[Key; 30]>, speeds: &Speeds, text: &Vec<Option<Key>>) -> Vec<[Key; 30]> {
    let candidates = candidates.into_par_iter().map(|candidate| {
        let speed = sim(&Layout::from_order(candidate), speeds, text);
        (candidate, speed)
    }).collect::<Vec<_>>();

    let mut top = vec![(Key::all(), INFINITY); 10];

    for (candidate, speed) in candidates {
        let bar = top.iter_mut().reduce(|x, y| { if x.1 > y.1 { x } else { y }}).unwrap();
        if speed < bar.1 {
            *bar = (candidate, speed);
        }
    }

    let fastest = top.iter().reduce(|x, y| if x.1 < y.1 { x } else { y }).unwrap();

    println!("fastest speed: {}, layout: {:?}", fastest.1, fastest.0);

    top.into_iter().map(|(x, _)| x).collect()
}

