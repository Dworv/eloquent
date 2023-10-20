use bevy::prelude::*;

pub fn root_ui() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..Default::default()
    }
}

pub fn row_container_ui(offset: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            left: offset,
            height: Val::Vh(10.),
            aspect_ratio: Some(10.),
            margin: UiRect::all(Val::Px(3.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(5.)),
            ..Default::default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..Default::default()
    }
}

pub fn add_key(builder: &mut ChildBuilder, key: char, font: Handle<Font>) {
    builder.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            aspect_ratio: Some(1.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // padding: UiRect::all(Val::Vh(2.5)),
            ..Default::default()
        },
        background_color: Color::rgb(0.2, 0.2, 0.5).into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            key,
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ).with_text_alignment(TextAlignment::Center));
    });
}
