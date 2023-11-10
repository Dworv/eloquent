mod speeds;

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

#[derive(Clone, Copy, Debug)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

impl Finger {
    pub fn index(&self) -> u8 {
        match self {
            Finger::LeftPinky => 0,
            Finger::LeftRing => 1,
            Finger::LeftMiddle => 2,
            Finger::LeftIndex => 3,
            Finger::RightIndex => 4,
            Finger::RightMiddle => 5,
            Finger::RightRing => 6,
            Finger::RightPinky => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot(u8);

impl Slot {
    pub fn new(row: u8, col: u8) -> Slot {
        assert!(row < 3);
        assert!(col < 10);
        Slot(row * 10 + col)
    }

    pub fn row(&self) -> u8 {
        self.0 / 10
    }

    pub fn col(&self) -> u8 {
        self.0 % 10
    }

    pub fn finger(&self) -> Finger {
        match self.col() {
            0 => Finger::LeftPinky,
            1 => Finger::LeftRing,
            2 => Finger::LeftMiddle,
            3 | 4 => Finger::LeftIndex,
            5 | 6 => Finger::RightIndex,
            7 => Finger::RightMiddle,
            8 => Finger::RightRing,
            9 => Finger::RightPinky,
            _ => panic!("Invalid column"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    keys: [Key; 30],
}

impl Layout {
    pub fn new(keys: [Key; 30]) -> Layout {
        Layout { keys }
    }

    pub fn key(&self, slot: Slot) -> Key {
        self.keys[slot.0 as usize]
    }

    pub fn slot(&self, key: Key) -> Slot {
        Slot(self.keys.iter().position(|&k| k == key).unwrap() as u8)
    }
}
