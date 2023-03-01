use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_proto::ProtoPlugin;
use bevy_rapier2d::prelude::*;
use resources::{CursorPos, SignsPool, TilesProperties, UiSettings, VelocityMultiplier};
use systems::PrototypSystemLabel;

mod components;
mod prototypes;
mod resources;
mod systems;
mod tiled;

use crate::systems::{animation, debug, movement, setup, sign, text};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("Prototyp"),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(tiled::TiledMapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ProtoPlugin::default())
        .insert_resource(UiSettings {
            show_debug_window: false,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(VelocityMultiplier(120.0))
        .init_resource::<CursorPos>()
        .init_resource::<TilesProperties>()
        .init_resource::<SignsPool>()
        .register_type::<TextureAtlasSprite>()
        .add_startup_system_to_stage(StartupStage::PreStartup, setup::startup)
        .add_startup_system(setup::spawn_camera)
        .add_startup_system(setup::spawn_player)
        .add_startup_system(text::spawn_fps_text)
        .add_system(debug::debug_input)
        .add_system(debug::draw_debug_ui)
        .add_system(debug::update_cursor_pos)
        .add_system(movement::player_movement.label(PrototypSystemLabel::Movement))
        .add_system(movement::camera_movement)
        .add_system(
            animation::update_character_animation
                .label(PrototypSystemLabel::UpdateAnimation)
                .after(PrototypSystemLabel::Movement),
        )
        .add_system(animation::animate.after(PrototypSystemLabel::UpdateAnimation))
        .add_system(text::update_fps_text)
        .add_system(sign::add_sign_sensors)
        .add_system(sign::handle_sign_collision.label(PrototypSystemLabel::SignUpdate))
        .add_system(sign::fix_sign_style.after(PrototypSystemLabel::SignUpdate))
        .run();
}
