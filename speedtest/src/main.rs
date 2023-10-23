use bevy::prelude::*;
use rand::{thread_rng, Rng};
use serde_json::json;
use speedtest::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NewTest>()
        .add_event::<NewSubTest>()
        .add_systems(Startup, (setup_camera, setup_keyboard, setup_tests))
        .add_systems(
            Update,
            (
                choose_next_test,
                update_key_backgrounds,
                choose_next_subtest,
                handle_key_presses.run_if(resource_exists::<CurrentSubTest>())
            ).chain(),
        )
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

fn choose_next_test(
    mut ev_new_test: EventReader<NewTest>,
    mut ev_new_subtest: EventWriter<NewSubTest>,
    mut remaining_tests: ResMut<RemainingTests>,
    mut current_test: ResMut<CurrentTest>,
    query: Query<(&Key, &ReactionTimes)>,
) {
    for _ in &mut ev_new_test {
        let old_len = remaining_tests.0.len();
        if old_len == 0 {
            let mut reaction_map = serde_json::Map::new();
            for (key, times) in query.iter() {
                reaction_map.insert(key.to_string(), json!(times.times));
            }
            println!("map: {:?}", reaction_map);
            std::process::exit(0);
        }
        let next_test = thread_rng().gen_range(0..old_len);
        *current_test = CurrentTest(Some(remaining_tests.0.get(next_test).unwrap().clone()));
        *remaining_tests = {
            let mut start = remaining_tests.0[0..next_test].to_vec();
            let mut end = remaining_tests.0[(next_test + 1)..old_len].to_vec();
            start.append(&mut end);
            RemainingTests(start)
        };
        ev_new_subtest.send(NewSubTest(1));
    }
}

fn update_key_backgrounds(
    mut ev_new_test: EventReader<NewTest>,
    mut query: Query<(&mut BackgroundColor, &Key)>,
    current_test: Res<CurrentTest>,
) {
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

fn choose_next_subtest(
    mut commands: Commands,
    mut ev_new_subtest: EventReader<NewSubTest>,
    test: Res<CurrentTest>,
    mut query: Query<(&mut BackgroundColor, &Key)>,
) {
    for num in &mut ev_new_subtest {
        let num = num.0;
        let test = test.0.as_ref().unwrap();
        let key_candidates = test.keys();
        let num_keys = key_candidates.len();
        let key1 = key_candidates[thread_rng().gen_range(0..num_keys)];
        let key2 = key_candidates[thread_rng().gen_range(0..num_keys)];
        commands.insert_resource(dbg!(CurrentSubTest {
            num,
            start: key1.clone(),
            end: key2,
            stage: SubTestStage::Preparing,
            start_time: 0.,
            wait_time: thread_rng().gen_range(1.0..3.0),
        }));
        for (mut bg, key) in &mut query {
            if key.keycode == key1 {
                dbg!(key.keycode, key1);
                *bg = PREP_KEY_COLOR;
            }
        } 
    } 
}

fn handle_key_presses(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut subtest: ResMut<CurrentSubTest>,
    test: Res<CurrentTest>,
    mut query: Query<(&mut BackgroundColor, &mut ReactionTimes, &Key)>,
    mut ev_new_subtest: EventWriter<NewSubTest>,
    mut ev_new_test: EventWriter<NewTest>
) {
    // dbg!(&subtest);
    match subtest.stage {
        SubTestStage::Preparing => {
            if input.just_pressed(subtest.start) {
                subtest.stage = SubTestStage::Waiting;
                subtest.start_time = time.elapsed_seconds_f64();
            }
        },
        SubTestStage::Waiting => {
            if time.elapsed_seconds_f64() - subtest.start_time > subtest.wait_time {
                subtest.stage = SubTestStage::Testing;
                subtest.start_time = time.elapsed_seconds_f64();
                for (mut bg, _, key) in &mut query {
                    if key.keycode == subtest.end {
                        *bg = PRESS_KEY_COLOR;
                    }
                }
            }
        },
        SubTestStage::Testing => {
            if input.just_pressed(subtest.end) {
                let elapsed = time.elapsed_seconds_f64() - subtest.start_time;
                println!("time {:?} -> {:?}: {}s", subtest.start, subtest.end, elapsed);
                for (mut bg, mut reaction_times, key) in &mut query {
                    if key.keycode == subtest.start {
                        *bg = ACTIVE_KEY_COLOR;
                        reaction_times.times.entry(subtest.end).or_insert_with(Vec::new).push(elapsed);
                        println!("reaction times: {:?}", reaction_times.times);
                    }
                    if key.keycode == subtest.end {
                        *bg = ACTIVE_KEY_COLOR;
                    }
                }
                if subtest.num == test.0.as_ref().unwrap().num_tests() {
                    ev_new_test.send(NewTest);
                } else {
                    ev_new_subtest.send(NewSubTest(subtest.num + 1));
                }
            }
        },
    }
}

