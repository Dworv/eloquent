use std::{fs::File, io::Read, f64::INFINITY};

use rand::prelude::*;
use rayon::prelude::*;
use sim::{Key, Layout, sim, Speeds, preprocess_str};

fn main() {
    
    for i in 0..100 {
        let (keys, speed) = anneal();
        println!("#{:03} speed:{:.0} layout:{:?}", i, speed, keys);
    }
}

fn anneal() -> ([Key; 30], f64) {
    let mut rng = thread_rng();
    let speeds = Speeds::init();
    let mut text = String::new();
    File::open("text/text.txt").unwrap().read_to_string(&mut text).unwrap();
    let text = preprocess_str(&text);

    let mut candidates: Vec<[Key; 30]> = Vec::new();
    for _ in 0..10 {
        let mut keys = Key::all();
        keys.shuffle(&mut rng);
        candidates.push(keys);
    }

    for i in 1..=100 {
        for og in 0..10 {
            for _ in 0..10 {
                let old = &candidates[og];
                let mut new = old.clone();
                for _ in 0..rng.gen_range(0..5) {
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
    }

    let mut x_speed = INFINITY;
    let fastest = candidates.into_iter().reduce(|x, y| {
        let y_speed = sim(&Layout::from_order(y), &speeds, &text);
        if y_speed < x_speed {
            x_speed = y_speed;
            y
        } else {
            x
        }
    });
    (fastest.unwrap(), x_speed)
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

    top.into_iter().map(|(x, _)| x).collect()
}

