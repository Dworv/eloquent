mod speeds;

use std::f64::NEG_INFINITY;

pub use speeds::Speeds;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Semicolon,
    Comma,
    Period,
    Slash,
}

impl Key {
    pub fn all() -> [Key; 30] {
        use Key::*;
        [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,P, Q, R, S, T, U, V, W, X, Y, Z, Semicolon, Comma, Period, Slash]
    }

    pub fn to_id(&self) -> u8 {
        match self {
            Key::A => 0,
            Key::B => 1,
            Key::C => 2,
            Key::D => 3,
            Key::E => 4,
            Key::F => 5,
            Key::G => 6,
            Key::H => 7,
            Key::I => 8,
            Key::J => 9,
            Key::K => 10,
            Key::L => 11,
            Key::M => 12,
            Key::N => 13,
            Key::O => 14,
            Key::P => 15,
            Key::Q => 16,
            Key::R => 17,
            Key::S => 18,
            Key::T => 19,
            Key::U => 20,
            Key::V => 21,
            Key::W => 22,
            Key::X => 23,
            Key::Y => 24,
            Key::Z => 25,
            Key::Semicolon => 26,
            Key::Comma => 27,
            Key::Period => 28,
            Key::Slash => 29,
        }
    }

    pub fn from_id(id: u8) -> Key {
        match id {
            0 => Key::A,
            1 => Key::B,
            2 => Key::C,
            3 => Key::D,
            4 => Key::E,
            5 => Key::F,
            6 => Key::G,
            7 => Key::H,
            8 => Key::I,
            9 => Key::J,
            10 => Key::K,
            11 => Key::L,
            12 => Key::M,
            13 => Key::N,
            14 => Key::O,
            15 => Key::P,
            16 => Key::Q,
            17 => Key::R,
            18 => Key::S,
            19 => Key::T,
            20 => Key::U,
            21 => Key::V,
            22 => Key::W,
            23 => Key::X,
            24 => Key::Y,
            25 => Key::Z,
            26 => Key::Semicolon,
            27 => Key::Comma,
            28 => Key::Period,
            29 => Key::Slash,
            _ => panic!("Invalid key id"),
        }
    }
}

impl TryFrom<char> for Key {
    type Error = &'static str;
    
    fn try_from(c: char) -> Result<Key, &'static str> {
        match c {
            'a' => Ok(Key::A),
            'b' => Ok(Key::B),
            'c' => Ok(Key::C),
            'd' => Ok(Key::D),
            'e' => Ok(Key::E),
            'f' => Ok(Key::F),
            'g' => Ok(Key::G),
            'h' => Ok(Key::H),
            'i' => Ok(Key::I),
            'j' => Ok(Key::J),
            'k' => Ok(Key::K),
            'l' => Ok(Key::L),
            'm' => Ok(Key::M),
            'n' => Ok(Key::N),
            'o' => Ok(Key::O),
            'p' => Ok(Key::P),
            'q' => Ok(Key::Q),
            'r' => Ok(Key::R),
            's' => Ok(Key::S),
            't' => Ok(Key::T),
            'u' => Ok(Key::U),
            'v' => Ok(Key::V),
            'w' => Ok(Key::W),
            'x' => Ok(Key::X),
            'y' => Ok(Key::Y),
            'z' => Ok(Key::Z),
            ';' => Ok(Key::Semicolon),
            ',' => Ok(Key::Comma),
            '.' => Ok(Key::Period),
            '/' => Ok(Key::Slash),
            _ => Err("invalid"),
        }
    }
}

pub fn row(slot: u8) -> u8 {
    slot / 10
}

pub fn col(slot: u8) -> u8 {
    slot % 10
}

pub fn finger(slot: u8) -> u8 {
    match col(slot) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 | 4 => 3,
        5 | 6 => 4,
        7 => 5,
        8 => 6,
        9 => 7,
        _ => panic!("Invalid column"),
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    slots: [u8; 30],
}

impl Layout {
    pub fn new(slots: [u8; 30]) -> Layout {
        Layout { slots }
    }

    pub fn from_order(keys: [Key; 30]) -> Layout {
        let mut slots = [0; 30];
        for (i, key) in keys.iter().enumerate() {
            slots[key.to_id() as usize] = i as u8;
        }
        Layout::new(slots)
    }

    pub fn slot(&self, key: Key) -> u8 {
        self.slots[key.to_id() as usize]
    }

    pub fn key(&self, slot: u8) -> Key {
        Key::from_id(self.slots.iter().position(|&s| s == slot).unwrap() as u8)
    }
}

#[derive(Clone, Copy, Debug)]
struct FingerState {
    slot: u8,
    time: f64,
}

pub fn preprocess_str(text: &str) -> Vec<Option<Key>> {
    let mut res = Vec::new();
    for c in text.chars() {
        if c == ' ' {
            res.push(None);
        } else if c == '\n' {
            continue;
        } else {
            if let Ok(key) = Key::try_from(c) {
                res.push(Some(key));
            }
        }
    }
    res
}

pub fn sim(layout: &Layout, speeds: &Speeds, text: &Vec<Option<Key>>) -> f64 {
    let mut timer = 0f64;
    let mut finger_states: [Option<FingerState>; 8] = [None; 8];

    for c in text {
        if let Some(key) = c {
            let slot = layout.slot(*key);
            let finger = finger(slot);
            let FingerState {
                slot: last_slot,
                time: last_time
            } = finger_states[finger as usize].unwrap_or(FingerState {
                slot,
                time: NEG_INFINITY,
            });

            let move_speed = speeds.time(last_slot, slot);
            let time_window = timer - last_time;

            if time_window < move_speed - speeds.min_time() {
                timer += move_speed - time_window;
            } else {
                timer += speeds.min_time();
            }

            finger_states[finger as usize] = Some(FingerState {
                slot,
                time: timer,
            });
        } else {
            timer += speeds.min_time();
        }
    }

    timer
}

#[cfg(test)]
mod tests {
    use crate::{Layout, Key, Speeds, preprocess_str};

    #[test]
    fn simple_test() {
        use Key::*;
        #[rustfmt::skip]
        let layout = Layout::from_order([Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Semicolon, Z, X, C, V, B, N, M, Comma, Period, Slash]);
        let speeds = Speeds::test_new([1.0; 8], [[1.0; 3]; 3]);
        assert_eq!(super::sim(&layout, &speeds, &preprocess_str("asdf")), 2.0);
    }

    #[test]
    fn complex_test() {
        use Key::*;
        #[rustfmt::skip]
        let layout = Layout::from_order([Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Semicolon, Z, X, C, V, B, N, M, Comma, Period, Slash]);
        let speeds = Speeds::test_new([1.0; 8], [[1.0; 3]; 3]);
        assert_eq!(super::sim(&layout, &speeds, &preprocess_str("qaz")), 2.5);
    }

    #[test]
    fn complex_test_2() {
        use Key::*;
        #[rustfmt::skip]
        let layout = Layout::from_order([Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Semicolon, Z, X, C, V, B, N, M, Comma, Period, Slash]);
        let speeds = Speeds::test_new([1.0; 8], [[1.0; 3]; 3]);
        assert_eq!(super::sim(&layout, &speeds, &preprocess_str("asdf")), 2.0);
    }

    #[test]
    fn big_test() {
        let layout = Layout::from_order(Key::all());
        let speeds = Speeds::test_new([0.5; 8], [[0.1, 0.2, 0.3]; 3]);
        let res = super::sim(&layout, &speeds, &preprocess_str("asdfalksdjflk;sdfsdfioqwpufjdsakvnl;ec fn ifg ifls  ukfcl gcimskjlfin\nasdfasdfalksdjflk;sdfsdfioqwpufjdsakvnl;ecfnifgiflsukfclgcimskjlfinasdfasdfalksdjflk ;sdfsdfioqw pufjdsakvnl;ecfnifgiflsukfclgcimskjlfinasdf"));
        assert!(res > 20.59 && res < 20.61);
    }
}