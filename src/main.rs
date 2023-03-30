use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_proto::ProtoPlugin;
use bevy_rapier2d::prelude::*;
use resources::{CursorPos, SignsPool, TilesProperties, UiSettings, NpcPool, VariablePool};
use systems::{PrototypSystemLabel, npc, collision::{self, PhysicsFilterTag, PlayerNpcContantFilter}};

mod components;
mod dialogue;
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
        .add_plugin(RapierPhysicsPlugin::<&PhysicsFilterTag>::pixels_per_meter(32.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ProtoPlugin::default())
        .insert_resource(PhysicsHooksWithQueryResource(Box::new(PlayerNpcContantFilter)))
        .insert_resource(UiSettings {
            show_debug_window: false,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .init_resource::<CursorPos>()
        .init_resource::<TilesProperties>()
        .init_resource::<SignsPool>()
        .init_resource::<NpcPool>()
        .init_resource::<VariablePool>()
        .register_type::<TextureAtlasSprite>()
        .register_type::<PhysicsFilterTag>()
        .register_type::<ActiveCollisionTypes>()
        .add_startup_system_to_stage(StartupStage::PreStartup, setup::startup)
        .add_startup_system(setup::spawn_camera)
        .add_startup_system(setup::spawn_player)
        .add_startup_system(text::spawn_fps_text)
        .add_system(npc::spawn_npcs.label(PrototypSystemLabel::SpawnNpcs))
        .add_system(npc::spawn_npc_dialogues.after(PrototypSystemLabel::SpawnNpcs))
        .add_system(systems::dialogue::resolve_dialogue)
        .add_system(debug::debug_input)
        .add_system(debug::draw_debug_ui)
        .add_system(debug::update_cursor_pos)
        .add_system(movement::player_movement.label(PrototypSystemLabel::Movement))
        .add_system(movement::ai_movement.label(PrototypSystemLabel::Movement))
        .add_system(movement::camera_movement)
        .add_system(collision::handle_player_npc_collision)
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