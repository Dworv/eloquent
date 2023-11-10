use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use sim::{Key, Layout, Slot, Speeds};

#[derive(Clone, Copy, Debug)]
struct FingerState {
    slot: Slot,
    time: f64,
}

fn main() {
    let speeds = Speeds::init();
    let buf = BufReader::new(File::open("text/text.txt").unwrap());

    #[rustfmt::skip]
    let layout = Layout::new([Key::Q, Key::W, Key::E, Key::R, Key::T, Key::Y, Key::U, Key::I, Key::O, Key::P, Key::A, Key::S, Key::D, Key::F, Key::G, Key::H, Key::J, Key::K, Key::L, Key::Semicolon, Key::Z, Key::X, Key::C, Key::V, Key::B, Key::N, Key::M, Key::Comma, Key::Period, Key::Slash]);
    let mut timer = 0f64;
    let mut finger_states: [Option<FingerState>; 8] = [None; 8];

    for c in buf
        .lines()
        .flatten()
        .flat_map(|s| s.chars().collect::<Vec<_>>())
    {
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

            dbg!(move_speed, time_window, timer);

            finger_states[finger.index() as usize] = Some(FingerState {
                slot,
                time: timer,
            });
        }
    }

    println!("time of qwerty: {} seconds", timer);
}
