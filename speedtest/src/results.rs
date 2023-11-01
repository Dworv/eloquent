use bevy::prelude::*;

use crate::{speedtest::TestLogs, AppState, ui::BACKGROUND_COLOR};

pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Results), setup_ui);
    }
}

fn setup_ui(mut commands: Commands, logs: Res<TestLogs>) {
    let mut fastest = std::f64::INFINITY;
    let mut sum = 0.;
    for log in &logs.0 {
        if log.time < fastest {
            fastest = log.time;
        }
        sum += log.time;
    }
    let avg = sum / logs.0.len() as f64;
    commands.spawn(NodeBundle {
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
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Fastest: {:.2}ms", fastest * 1000.),
            TextStyle {
                font_size: 30.,
                ..default()
            },
        ));
        parent.spawn(TextBundle::from_section(
            format!("\nAverage: {:.2}ms", avg * 1000.),
            TextStyle {
                font_size: 30.,
                ..default()
            },
        ));
    });
}