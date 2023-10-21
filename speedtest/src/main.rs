use bevy::prelude::*;
use rand::{thread_rng, Rng};
use speedtest::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NewTest>()
        .add_systems(Startup, (setup_camera, setup_keyboard, setup_tests))
        .add_systems(Update, (choose_next_test, update_key_backgrounds).chain())
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

fn setup_tests(mut commands: Commands, mut ev_new_test: EventWriter<NewTest>) {
    commands.insert_resource(RemainingTests::default());
    commands.init_resource::<CurrentTest>();
    ev_new_test.send(NewTest);
}

fn choose_next_test(mut ev_new_test: EventReader<NewTest>, mut remaining_tests: ResMut<RemainingTests>, mut current_test: ResMut<CurrentTest>) {
    for _ in &mut ev_new_test {
        let old_len = remaining_tests.0.len();
        let next_test = thread_rng().gen_range(0..old_len);
        *current_test = CurrentTest(Some(remaining_tests.0.get(next_test).unwrap().clone()));
        *remaining_tests = {
            let mut start = remaining_tests.0[0..next_test].to_vec();
            let mut end = remaining_tests.0[(next_test + 1)..old_len].to_vec();
            start.append(&mut end);
            RemainingTests(start)
        };
        println!("current test: {:?}", current_test.0.as_ref().unwrap().home_key());
    }
}

fn update_key_backgrounds(mut ev_new_test: EventReader<NewTest>, mut query: Query<(&mut BackgroundColor, &Key)>, current_test: Res<CurrentTest>) {
    let highlighted_keys = current_test.0.as_ref().unwrap().keys();
    for _ in &mut ev_new_test {
        for (mut bg, key) in &mut query {
            if highlighted_keys.contains(&key.keycode) {
                *bg = ACTIVE_KEY_COLOR;
            } else {
                *bg = INACTIVE_KEY_COLOR;
            }
        }
    }
}