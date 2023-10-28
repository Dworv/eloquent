use bevy::prelude::*;

pub const BACKGROUND_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.1,
    green: 0.1,
    blue: 0.1,
    alpha: 1.0,
});

pub const NORMAL_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.3,
    green: 0.3,
    blue: 0.3,
    alpha: 1.0,
});

pub const HAND_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.2,
    green: 0.2,
    blue: 0.5,
    alpha: 1.0,
});

pub const START_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.2,
    green: 0.5,
    blue: 0.2,
    alpha: 1.0,
});

pub const END_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.5,
    green: 0.2,
    blue: 0.5,
    alpha: 1.0,
});

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
    builder
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                aspect_ratio: Some(1.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: NORMAL_KEY_COLOR,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    key,
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}
