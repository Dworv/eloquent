pub mod instructions;
pub mod speedtest;
pub mod ui;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Instructions,
    Test,
    Results,
}

#[derive(Component)]
pub struct Key(KeyCode);

impl From<Key> for char {
    fn from(value: Key) -> Self {
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
            _ => panic!("invalid key")
        }
    }
}