pub mod instructions;
pub mod speedtest;
pub mod ui;

use bevy::prelude::*;
use rand::Rng;
use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Instructions,
    Test,
    Results,
}

#[derive(Clone, Component, Debug, PartialEq)]
pub struct Key(KeyCode);

impl From<&Key> for char {
    fn from(value: &Key) -> Self {
        use KeyCode::*;
        match value.0 {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
            I => 'i',
            J => 'j',
            K => 'k',
            L => 'l',
            M => 'm',
            N => 'n',
            O => 'o',
            P => 'p',
            Q => 'q',
            R => 'r',
            S => 's',
            T => 't',
            U => 'u',
            V => 'v',
            W => 'w',
            X => 'x',
            Y => 'y',
            Z => 'z',
            Semicolon => ';',
            Comma => ',',
            Period => '.',
            Slash => '/',
            _ => panic!("invalid key"),
        }
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_char(char::from(self))
    }
}

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
    pub fn random() -> Finger {
        use Finger::*;
        match rand::thread_rng().gen_range(0..8) {
            0 => LeftPinky,
            1 => LeftRing,
            2 => LeftMiddle,
            3 => LeftIndex,
            4 => RightIndex,
            5 => RightMiddle,
            6 => RightRing,
            7 => RightPinky,
            _ => panic!("invalid finger"),
        }
    }

    pub fn keys(&self) -> Vec<Key> {
        use Finger::*;
        use KeyCode::*;
        match self {
            LeftPinky => vec![Key(Q), Key(A), Key(Z)],
            LeftRing => vec![Key(W), Key(S), Key(X)],
            LeftMiddle => vec![Key(E), Key(D), Key(C)],
            LeftIndex => vec![Key(R), Key(F), Key(V), Key(T), Key(G), Key(B)],
            RightIndex => vec![Key(Y), Key(H), Key(N), Key(U), Key(J), Key(M)],
            RightMiddle => vec![Key(I), Key(K), Key(Comma)],
            RightRing => vec![Key(O), Key(L), Key(Period)],
            RightPinky => vec![Key(P), Key(Semicolon), Key(Slash)],
        }
    }
}
