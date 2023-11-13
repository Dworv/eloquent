use std::{
    fs::File,
    io::Read,
};

use sim::{Key, Layout, Slot, Speeds};

#[derive(Clone, Copy, Debug)]
struct FingerState {
    slot: Slot,
    time: f64,
}

fn sim(layout: &Layout, speeds: &Speeds, text: &str) -> f64 {
    let mut timer = 0f64;
    let mut finger_states: [Option<FingerState>; 8] = [None; 8];

    for c in text.chars() {
        if c == ' ' {
            timer += speeds.space_time();
            continue;
        }
        if c == '\n' {
            continue;
        }
        if let Ok(key) = Key::try_from(c) {
            let slot = layout.slot(key);
            let finger = slot.finger();
            let FingerState {
                slot: last_slot,
                time: last_time
            } = finger_states[finger.index() as usize].unwrap_or(FingerState {
                slot: Slot::new(slot.row(), slot.row()),
                time: 0f64,
            });

            let move_speed = speeds.time(last_slot, slot);
            let time_window = timer - last_time;

            if time_window < move_speed {
                timer += move_speed - time_window;
            }

            finger_states[finger.index() as usize] = Some(FingerState {
                slot,
                time: timer,
            });
        }
    }

    timer
}

fn main() {
    let speeds = Speeds::init();
    let mut txt = String::new();
    File::open("text/text.txt").unwrap().read_to_string(&mut txt).unwrap();

    #[rustfmt::skip]
    use Key::*;
    let qwerty = Layout::new([Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Semicolon, Z, X, C, V, B, N, M, Comma, Period, Slash]);
    let dvorak = Layout::new([Slash, Comma, Period, P, Y, F, G, C, R, L, A, O, E, U, I, D, H, T, N, S, Semicolon, Q, J, K, X, B, M, W, V, Z]); 

    println!("time of qwerty: {} seconds", sim(&qwerty, &speeds, &txt));
    println!("time of dvorak: {} seconds", sim(&dvorak, &speeds, &txt));
}
