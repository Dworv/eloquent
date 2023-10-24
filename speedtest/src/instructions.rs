use bevy::prelude::*;
use crate::{AppState, ui::BACKGROUND_COLOR};

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Instructions), setup_ui);
    }
}

#[derive(Component)]
struct InstructionsUi;

fn setup_ui(mut commands: Commands) {
    println!("hi");
    commands.spawn((
        InstructionsUi,
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
    ));
}

fn handle_ui_events(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    todo!()
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    todo!()
}