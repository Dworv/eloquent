mod speeds;

pub use speeds::Speeds;

#[derive(Clone, Copy, Debug)]
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
    Slash
}

#[derive(Clone, Copy, Debug)]
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
}

#[derive(Clone, Debug)]
pub struct Layout {
    keys: [Key; 30]
}

impl Layout {
    pub fn new(keys: [Key; 30]) -> Layout {
        Layout { keys }
    }

    pub fn key(&self, slot: Slot) -> Key {
        self.keys[slot.0 as usize]
    }
}