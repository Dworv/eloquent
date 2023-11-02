use crate::{ui::BACKGROUND_COLOR, AppState};
use bevy::prelude::*;

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Instructions), setup_ui)
            .add_systems(Update, handle_next.run_if(in_state(AppState::Instructions)))
            .add_systems(OnExit(AppState::Instructions), destroy_ui);
    }
}

#[derive(Component)]
struct InstructionsUi;

#[derive(Component)]
struct InstructionsText;

#[derive(Resource)]
enum InstructionsStage {
    Title,
    P1,
    P2,
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            InstructionsUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Percent(15.)),
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
            parent.spawn((
                TextBundle::from_section(
                    "Instructions",
                    TextStyle {
                        font_size: 30.,
                        ..default()
                    },
                ),
                InstructionsText,
            ));
            parent.spawn(TextBundle::from_section(
                "\n[Press space to continue]",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ));
        });
    commands.insert_resource(InstructionsStage::Title);
}

fn handle_next(
    mut query: Query<&mut Text, With<InstructionsText>>,
    input: Res<Input<KeyCode>>,
    mut stage: ResMut<InstructionsStage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let instructions = &mut query.single_mut().sections[0].value;
        match *stage {
            InstructionsStage::Title => {
                *instructions =
                    "To use the test, place and hold your appropriate finger on the green key."
                        .to_string();
                *stage = InstructionsStage::P1;
            }
            InstructionsStage::P1 => {
                *instructions = "A red key will appear. When you're ready, move the held finger to the new key as fast as possible.".to_string();
                *stage = InstructionsStage::P2;
            }
            InstructionsStage::P2 => {
                *next_state = NextState(Some(AppState::Test));
            }
        }
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<InstructionsUi>>) {
    commands.entity(query.single()).despawn();
}
