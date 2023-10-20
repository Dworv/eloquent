use bevy::prelude::*;

use speedtest::{add_key, root_ui, row_container_ui, Key};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_keyboard))
        .add_systems(Update, highlight_key)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_keyboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(root_ui()).with_children(|parent| {
        parent
            .spawn(row_container_ui(Val::Vh(0.)))
            .with_children(|parent| {
                add_key(parent, 'Q', font.clone());
                add_key(parent, 'W', font.clone());
                add_key(parent, 'E', font.clone());
                add_key(parent, 'R', font.clone());
                add_key(parent, 'T', font.clone());
                add_key(parent, 'Y', font.clone());
                add_key(parent, 'U', font.clone());
                add_key(parent, 'I', font.clone());
                add_key(parent, 'O', font.clone());
                add_key(parent, 'P', font.clone());
            });
        parent
            .spawn(row_container_ui(Val::Vh(2.5)))
            .with_children(|parent| {
                add_key(parent, 'A', font.clone());
                add_key(parent, 'S', font.clone());
                add_key(parent, 'D', font.clone());
                add_key(parent, 'F', font.clone());
                add_key(parent, 'G', font.clone());
                add_key(parent, 'H', font.clone());
                add_key(parent, 'J', font.clone());
                add_key(parent, 'K', font.clone());
                add_key(parent, 'L', font.clone());
                add_key(parent, ';', font.clone());
            });
        parent
            .spawn(row_container_ui(Val::Vh(7.5)))
            .with_children(|parent| {
                add_key(parent, 'Z', font.clone());
                add_key(parent, 'X', font.clone());
                add_key(parent, 'C', font.clone());
                add_key(parent, 'V', font.clone());
                add_key(parent, 'B', font.clone());
                add_key(parent, 'N', font.clone());
                add_key(parent, 'M', font.clone());
                add_key(parent, ',', font.clone());
                add_key(parent, '.', font.clone());
                add_key(parent, '/', font.clone());
            });
    });
}

fn highlight_key(mut query: Query<(&mut BackgroundColor, &Key)>, input: Res<Input<KeyCode>>) {
    for (mut color, key) in &mut query {
        if input.just_pressed(key.keycode) {
            println!("key pressed: {:?}", key.keycode);
            color.0 = Color::rgb(0.5, 0.2, 0.5);
        }
        if input.just_released(key.keycode) {
            color.0 = Color::rgb(0.2, 0.2, 0.5);
        }
    }
}
