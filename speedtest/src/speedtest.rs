use bevy::prelude::*;

use crate::{ui::{BACKGROUND_COLOR, row_container_ui, add_key}, AppState, Key};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Test), setup_ui);
    }
}

#[derive(Component)]
struct TestUi;

#[derive(Component)]
struct StartKey;

#[derive(Component)]
struct EndKey;

#[derive(Resource)]
struct TestsRemaining(pub u32);

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
        )).with_children(|parent| {
            parent.spawn(row_container_ui(Val::Px(0.))).with_children(|parent| {
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
            parent.spawn(row_container_ui(Val::Px(20.))).with_children(|parent| {
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
            parent.spawn(row_container_ui(Val::Px(60.))).with_children(|parent| {
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