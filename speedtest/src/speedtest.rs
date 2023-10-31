use std::{fs::OpenOptions, time::{Instant, SystemTime, UNIX_EPOCH}};

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;

use crate::{
    ui::{
        add_key, row_container_ui, ACTIVE_KEY_COLOR, BACKGROUND_COLOR, END_KEY_COLOR,
        NORMAL_KEY_COLOR, START_KEY_COLOR,
    },
    AppState, Finger, Key,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NextTest>()
            .add_state::<TestState>()
            .add_systems(OnEnter(AppState::Test), (setup_ui, activate_first_test, init_res))
            .add_systems(
                Update,
                (
                    color_start_key,
                    color_end_key,
                    color_active_keys,
                    color_normal_keys,
                )
                    .run_if(in_state(AppState::Test)),
            )
            .add_systems(
                Update,
                (
                    handle_start.run_if(in_state(TestState::Waiting)),
                    handle_lift.run_if(in_state(TestState::Started)),
                    handle_finish.run_if(in_state(TestState::Moving))
                )
                    .run_if(in_state(AppState::Test)),
            )
            .add_systems(
                Update,
                (log_test, remaining_check, clear_old_test, choose_new_test)
                    .chain()
                    .run_if(on_event::<NextTest>()),
            )
            .add_systems(OnExit(AppState::Test), cleanup_test);
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum TestState {
    #[default]
    Waiting,
    Started,
    Moving,
}

#[derive(Component)]
struct TestUi;

#[derive(Component)]
struct ActiveKey;

#[derive(Component)]
struct StartKey;

#[derive(Component)]
struct EndKey;

#[derive(Resource)]
struct TestsRemaining(pub i32);

#[derive(Resource)]
pub struct TestLogs(pub Vec<TestLog>);

#[derive(Resource)]
struct LiftTime(f64);

#[derive(Debug, Serialize)]
pub struct TestLog {
    pub start: Key,
    pub end: Key,
    pub time: f64,
}

#[derive(Event)]
struct NextTest;

fn init_res(mut commands: Commands) {
    commands.insert_resource(TestsRemaining(3));
    commands.insert_resource(TestLogs(Vec::new()));
    commands.insert_resource(LiftTime(0.));
}

fn setup_ui(mut commands: Commands) {
    use KeyCode::*;
    commands
        .spawn((
            TestUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BACKGROUND_COLOR,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(row_container_ui(Val::Px(0.)))
                .with_children(|parent| {
                    add_key(parent, Key(Q));
                    add_key(parent, Key(W));
                    add_key(parent, Key(E));
                    add_key(parent, Key(R));
                    add_key(parent, Key(T));
                    add_key(parent, Key(Y));
                    add_key(parent, Key(U));
                    add_key(parent, Key(I));
                    add_key(parent, Key(O));
                    add_key(parent, Key(P));
                });
            parent
                .spawn(row_container_ui(Val::Px(15.)))
                .with_children(|parent| {
                    add_key(parent, Key(A));
                    add_key(parent, Key(S));
                    add_key(parent, Key(D));
                    add_key(parent, Key(F));
                    add_key(parent, Key(G));
                    add_key(parent, Key(H));
                    add_key(parent, Key(J));
                    add_key(parent, Key(K));
                    add_key(parent, Key(L));
                    add_key(parent, Key(Semicolon));
                });
            parent
                .spawn(row_container_ui(Val::Px(45.)))
                .with_children(|parent| {
                    add_key(parent, Key(Z));
                    add_key(parent, Key(X));
                    add_key(parent, Key(C));
                    add_key(parent, Key(V));
                    add_key(parent, Key(B));
                    add_key(parent, Key(N));
                    add_key(parent, Key(M));
                    add_key(parent, Key(Comma));
                    add_key(parent, Key(Period));
                    add_key(parent, Key(Slash));
                });
        });
}

fn color_start_key(mut start: Query<&mut BackgroundColor, With<StartKey>>, state: Res<State<TestState>>) {
    for mut color in &mut start {
        *color = match **state {
            TestState::Waiting | TestState::Started => START_KEY_COLOR,
            TestState::Moving => ACTIVE_KEY_COLOR,
        }
    }
}

fn color_end_key(mut end: Query<&mut BackgroundColor, With<EndKey>>, state: Res<State<TestState>>) {
    for mut color in &mut end {
        *color = match **state {
            TestState::Waiting => ACTIVE_KEY_COLOR,
            TestState::Started | TestState::Moving => END_KEY_COLOR,
        }
    }
}

fn color_active_keys(mut active: Query<&mut BackgroundColor, With<ActiveKey>>) {
    for mut color in &mut active {
        *color = ACTIVE_KEY_COLOR;
    }
}

fn color_normal_keys(
    mut normal: Query<
        &mut BackgroundColor,
        (
            With<Key>,
            Without<StartKey>,
            Without<EndKey>,
            Without<ActiveKey>,
        ),
    >,
) {
    for mut color in &mut normal {
        *color = NORMAL_KEY_COLOR;
    }
}

fn handle_start(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<TestState>>,
    start_key: Query<&Key, With<StartKey>>,
) {
    for start in &start_key {
        if input.just_pressed(start.0) {
            *next_state = NextState(Some(TestState::Started));
        }
    }
}

fn handle_lift(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<TestState>>,
    start_key: Query<&Key, With<StartKey>>,
    mut start_time: ResMut<LiftTime>,
    time: Res<Time>,
) {
    for start in &start_key {
        if input.just_released(start.0) {
            start_time.0 = time.elapsed_seconds_f64();
            *next_state = NextState(Some(TestState::Moving));
        }
    }
}

fn handle_finish(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<TestState>>,
    end_key: Query<&Key, With<EndKey>>,
    mut ev: EventWriter<NextTest>,
) {
    for end in &end_key {
        if input.just_pressed(end.0) {
            *next_state = NextState(Some(TestState::Waiting));
            ev.send(NextTest);
        }
    }
}

fn activate_first_test(mut ev: EventWriter<NextTest>) {
    ev.send(NextTest);
}

fn log_test(
    mut test_logs: ResMut<TestLogs>,
    start_key: Query<&Key, With<StartKey>>,
    end_key: Query<&Key, With<EndKey>>,
    start_time: Res<LiftTime>,
    time: Res<Time>,
) {
    for start in &start_key {
        for end in &end_key {
            let time = time.elapsed_seconds_f64() - start_time.0;
            test_logs.0.push(TestLog {
                start: start.clone(),
                end: end.clone(),
                time,
            });
        }
    }
}

fn clear_old_test(
    mut commands: Commands,
    // mut tests_remaining: ResMut<TestsRemaining>,
    // mut test_logs: ResMut<TestLogs>,
    query: Query<Entity, Or<(With<StartKey>, With<EndKey>, With<ActiveKey>)>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<StartKey>()
            .remove::<EndKey>()
            .remove::<ActiveKey>();
    }
}

fn remaining_check(
    mut tests_remaining: ResMut<TestsRemaining>,
    mut next_state: ResMut<NextState<AppState>>,
    log: Res<TestLogs>,
) {
    if tests_remaining.0 == 0 {
        let file = OpenOptions::new().create(true).write(true).open(format!("speedtest-{}.json", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())).unwrap();
        serde_json::to_writer_pretty(file, &log.0).unwrap();
        *next_state = NextState(Some(AppState::Results));
    }
    tests_remaining.0 -= 1;
}

fn choose_new_test(mut commands: Commands, mut keys: Query<(Entity, &Key)>) {
    let finger = Finger::random();
    let mut fkeys = finger.keys();
    fkeys.shuffle(&mut thread_rng());
    let start = fkeys.pop().unwrap();
    let end = fkeys.pop().unwrap();
    let actives = fkeys;
    for (entity, key) in keys.iter_mut() {
        if *key == start {
            commands.entity(entity).insert(StartKey);
        }
        if *key == end {
            commands.entity(entity).insert(EndKey);
        }
        if actives.contains(key) {
            commands.entity(entity).insert(ActiveKey);
        }
    }
}

fn cleanup_test(mut commands: Commands, query: Query<Entity, With<TestUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
