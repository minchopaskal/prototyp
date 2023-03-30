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

pub enum TextBuilder<'w, 's, 'a, 'c> {
    Commands(&'a mut Commands<'w, 's>),
    Parent(&'c mut ChildBuilder<'w, 's, 'a>)
}

// @param visibile We can hide the text at first,
// as we might want to move it depending on it's size.
pub fn spawn_text<T: Component>(
    commands: TextBuilder,
    asset_server: &Res<AssetServer>,
    text: Vec<TextValue>,
    text_pos: TextPosition,
    visible: bool,
    split: bool,
    marker: Option<T>,
) -> Entity {
    let mut text_sections: Vec<TextSection> = text
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

    let mut split_sections = Vec::new();
    if split {
        for section in text_sections {
            for value in section.value.split(" ") {
                split_sections.push(TextSection {
                    value: value.to_string(),
                    style: section.style.clone(),
                })
            }
        }
        text_sections = split_sections;
    }

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

    let id;
    match commands {
    TextBuilder::Commands(commands) => {
        id = commands
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
    },
    TextBuilder::Parent(commands) => {
        let font_size = if !text_sections.is_empty() {
            text_sections[0].style.font_size
        } else {
            0.0
        };

        // Id doesn't matter - parent will take care of despawing
        id = Entity::from_raw(0);

        if split {
            for section in text_sections {
                commands.spawn(TextBundle::from_section(section.value, section.style)
                    .with_style(Style {
                        max_size: Size::new(Val::Undefined, Val::Px(font_size)),
                        margin: UiRect::all(Val::Percent(1.0)),
                        ..Default::default()
                    })
                );
            }
        } else {
            commands
                .spawn(TextBundle::from_sections(text_sections)
                .with_style(Style {
                        position_type: PositionType::Absolute,
                        position,
                        max_size: Size::new(Val::Undefined, Val::Px(font_size)),
                        margin: UiRect::all(Val::Percent(1.0)),
                        ..Default::default()
                    })
                );
        } 
        
        commands.spawn(NodeBundle {
            style: Style {
                flex_grow: 2.,
                ..default()
            },
            ..default()
        });
    },
    }

    id
}

pub fn spawn_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_text(
        TextBuilder::Commands(&mut commands),
        &asset_server,
        vec![TextValue::Name("FPS:"), TextValue::Debug("0.0")],
        TextPosition::Percent(0, 5),
        true,
        false,
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
    

    commands
        .entity(dialogue_box_entt).with_children(|parent| {
            spawn_text::<T>(
                TextBuilder::Parent(parent),
                asset_server,
                vec![TextValue::Dialogue(&format!("{name}:"))],
                TextPosition::Percent(1, 10),
                true,
                false,
                name_marker,
            );
        
            println!("Text: {text}");

            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(80.0)),
                    position: UiRect {
                        top: Val::Percent(30.0),
                        left: Val::Percent(0.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                spawn_text::<T>(
                    TextBuilder::Parent(p),
                    asset_server,
                    vec![TextValue::Dialogue(text)],
                    TextPosition::Percent(0, 0), // doesn't matter in this case. Probably should split functions at this point
                    true,
                    true,
                    marker,
                );
            });
        })
        .id()
}
