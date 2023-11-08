use bevy::prelude::*;
use speedtest::{
    instructions::InstructionsPlugin, results::ResultsPlugin, speedtest::TestPlugin, AppState,
};

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(InstructionsPlugin)
        .add_plugins(TestPlugin)
        .add_plugins(ResultsPlugin)
        .add_systems(Startup, setup_camera)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
