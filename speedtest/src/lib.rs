use bevy::prelude::*;

pub const INACTIVE_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 });
pub const ACTIVE_KEY_COLOR: BackgroundColor = BackgroundColor(Color::Rgba { red: 0.2, green: 0.2, blue: 0.5, alpha: 1.0 });

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
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    aspect_ratio: Some(1.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: INACTIVE_KEY_COLOR,
                ..Default::default()
            },
            Key {
                keycode: char_to_keycode(key),
            },
        ))
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

#[derive(Component)]
pub struct Key {
    pub keycode: KeyCode,
}

pub fn char_to_keycode(c: char) -> KeyCode {
    match c {
        'A' => KeyCode::A,
        'B' => KeyCode::B,
        'C' => KeyCode::C,
        'D' => KeyCode::D,
        'E' => KeyCode::E,
        'F' => KeyCode::F,
        'G' => KeyCode::G,
        'H' => KeyCode::H,
        'I' => KeyCode::I,
        'J' => KeyCode::J,
        'K' => KeyCode::K,
        'L' => KeyCode::L,
        'M' => KeyCode::M,
        'N' => KeyCode::N,
        'O' => KeyCode::O,
        'P' => KeyCode::P,
        'Q' => KeyCode::Q,
        'R' => KeyCode::R,
        'S' => KeyCode::S,
        'T' => KeyCode::T,
        'U' => KeyCode::U,
        'V' => KeyCode::V,
        'W' => KeyCode::W,
        'X' => KeyCode::X,
        'Y' => KeyCode::Y,
        'Z' => KeyCode::Z,
        ';' => KeyCode::Semicolon,
        ',' => KeyCode::Comma,
        '.' => KeyCode::Period,
        '/' => KeyCode::Slash,
        _ => panic!("no keycode???? :O")
    }
}

#[derive(Clone)]
pub enum Speedtest {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

impl Speedtest {
    pub fn home_key(&self) -> KeyCode {
        use KeyCode::*;
        use Speedtest::*;
        match self {
            LeftPinky => A,
            LeftRing => S,
            LeftMiddle => D,
            LeftIndex => F,
            RightIndex => J,
            RightMiddle => K,
            RightRing => L,
            RightPinky => Semicolon,
        }
    }

    pub fn keys(&self) -> Vec<KeyCode> {
        use KeyCode::*;
        use Speedtest::*;
        match self {
            LeftPinky => vec![Q, A, Z],
            LeftRing => vec![W, S, X],
            LeftMiddle => vec![E, D, C],
            LeftIndex => vec![R, F, V, T, G, B],
            RightIndex => vec![Y, H, N, U, J, M],
            RightMiddle => vec![I, K, Comma],
            RightRing => vec![O, L, Period],
            RightPinky => vec![P, Semicolon, Slash],
        }
    }
}

#[derive(Default, Resource)]
pub struct CurrentTest(pub Option<Speedtest>);

#[derive(Resource)]
pub struct RemainingTests(pub Vec<Speedtest>);

impl Default for RemainingTests {
    fn default() -> Self {
        use Speedtest::*;
        Self(vec![
            LeftPinky,
            LeftRing,
            LeftMiddle,
            LeftIndex,
            RightIndex,
            RightMiddle,
            RightRing,
            RightPinky,
        ])
    }
}

#[derive(Event)]
pub struct NewTest;