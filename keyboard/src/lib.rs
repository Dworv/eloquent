pub struct KeyboardLayout {
    pub keys: [[Key; 10]; 3],
}

impl KeyboardLayout {
    pub fn qwerty() -> Self {
        Self {
            keys: [
                [
                    Key::Q,
                    Key::W,
                    Key::E,
                    Key::R,
                    Key::T,
                    Key::Y,
                    Key::U,
                    Key::I,
                    Key::O,
                    Key::P,
                ],
                [
                    Key::A,
                    Key::S,
                    Key::D,
                    Key::F,
                    Key::G,
                    Key::H,
                    Key::J,
                    Key::K,
                    Key::L,
                    Key::Semicolon,
                ],
                [
                    Key::Z,
                    Key::X,
                    Key::C,
                    Key::V,
                    Key::B,
                    Key::N,
                    Key::M,
                    Key::Comma,
                    Key::Period,
                    Key::Slash,
                ],
            ],
        }
    }
}

pub struct KeyboardMap<T> {
    pub keys: [[T; 10]; 3],
    pub layout: KeyboardLayout,
}

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
