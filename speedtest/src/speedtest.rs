use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

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
            .add_systems(OnEnter(AppState::Test), (setup_ui, activate_first_test))
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
                (clear_old_test, choose_new_test)
                    .chain()
                    .run_if(on_event::<NextTest>()),
            );
    }
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
struct TestsRemaining(pub u32);

#[derive(Resource)]
struct TestLogs(pub Vec<TestLog>);

struct TestLog {
    start: Key,
    end: Key,
    time: f64,
}

#[derive(Event)]
struct NextTest;

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

fn color_start_key(mut start: Query<&mut BackgroundColor, With<StartKey>>) {
    for mut color in &mut start {
        *color = START_KEY_COLOR;
    }
}

fn color_end_key(mut end: Query<&mut BackgroundColor, With<EndKey>>) {
    for mut color in &mut end {
        *color = END_KEY_COLOR;
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

fn activate_first_test(mut ev: EventWriter<NextTest>) {
    ev.send(NextTest);
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
