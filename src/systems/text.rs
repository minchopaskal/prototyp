use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::components::FPSTextMarker;

pub enum TextValue<'a> {
    Name(&'a str),
    Dialogue(&'a str),
    Debug(&'a str),
}

pub enum TextPosition {
    Percent(u8, u8),
    Absolute(f32, f32),
}

// @param visibile We can hide the text at first,
// as we might want to move it depending on it's size.
pub fn spawn_text<T: Component>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: Vec<TextValue>,
    text_pos: TextPosition,
    visible: bool,
    marker: Option<T>,
) -> Entity {
    let text_sections: Vec<TextSection> = text
        .iter()
        .map(|text| {
            let bundle = match text {
                TextValue::Name(val) => (
                    val,
                    TextStyle {
                        font: asset_server.load("fonts/pixelboy.ttf"),
                        font_size: 64.0,
                        color: Color::rgb(0.0, 0.0, 1.0),
                    },
                ),
                TextValue::Dialogue(val) => (
                    val,
                    TextStyle {
                        font: asset_server.load("fonts/pixelboy.ttf"),
                        font_size: 32.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ),
                TextValue::Debug(val) => (
                    val,
                    TextStyle {
                        font: asset_server.load("fonts/vera_mono.ttf"),
                        font_size: 64.0,
                        color: Color::rgb(1.0, 0.0, 0.0),
                    },
                ),
            };

            TextSection {
                value: bundle.0.to_string(),
                style: bundle.1,
            }
        })
        .collect();

    let position = match text_pos {
        TextPosition::Percent(x, y) => UiRect {
            left: Val::Percent(x as f32),
            top: Val::Percent(y as f32),
            ..Default::default()
        },
        TextPosition::Absolute(x, y) => UiRect {
            left: Val::Px(x),
            top: Val::Px(y),
            ..Default::default()
        },
    };

    let id = commands
        .spawn(TextBundle::from_sections(text_sections).with_style(Style {
            position_type: PositionType::Absolute,
            position,
            ..Default::default()
        }))
        .id();

    if let Some(m) = marker {
        commands.entity(id).insert(m);
    }

    if !visible {
        commands.entity(id).insert(Visibility { is_visible: false });
    }

    id
}

pub fn spawn_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_text(
        &mut commands,
        &asset_server,
        vec![TextValue::Name("FPS:"), TextValue::Debug("0.0")],
        TextPosition::Percent(0, 5),
        true,
        Some(FPSTextMarker),
    );
}

pub fn update_fps_text(
    diagnostics: Res<Diagnostics>,
    mut text_q: Query<&mut Text, With<FPSTextMarker>>,
) {
    for mut text in text_q.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn spawn_dialog_box<T: Component + Default>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    name: &str,
    text: &str,
    marker: Option<T>,
) -> Entity {
    let dialogue_box_entt = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(30.0)),
                position: UiRect {
                    top: Val::Percent(60.0),
                    left: Val::Percent(10.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.2, 0.4).into(),
            ..default()
        })
        .id();

    // "Copy" the marker
    // TODO: check if we can bound T to be unit type(without user defined trait)
    let name_marker = if marker.is_some() {
        Some(T::default())
    } else {
        None
    };
    let name_entt = spawn_text::<T>(
        commands,
        asset_server,
        vec![TextValue::Dialogue(&format!("{name}:"))],
        TextPosition::Percent(10, 10),
        true,
        name_marker,
    );

    let text_entt = spawn_text::<T>(
        commands,
        asset_server,
        vec![TextValue::Dialogue(text)],
        TextPosition::Percent(10, 30),
        true,
        marker,
    );

    commands
        .entity(dialogue_box_entt)
        .add_child(name_entt)
        .add_child(text_entt)
        .id()
}
