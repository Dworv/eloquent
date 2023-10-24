pub mod instructions;
pub mod ui;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Instructions,
    Test,
    Results
}