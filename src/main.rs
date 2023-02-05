use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier2d::prelude::*;
use components::{Animation, SpriteFlip};
use resources::{CursorPos, TilesProperties, VelocityMultiplier};
use systems::PrototypSystemLabel;

mod components;
mod resources;
mod systems;
mod tiled;

use crate::components::{AnimationState, Player};
use crate::systems::{animation, cursor_pos, debug, movement};

#[derive(Resource)]
struct ShowWindow(bool);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<tiled::TiledMap> = asset_server.load("map.tmx");

    commands.spawn(tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlasses: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load("cat.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.0), 8, 38, None, None);

    let atlas_handle = atlasses.add(atlas);

    let sprite = TextureAtlasSprite::new(4);

    commands
        .spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(20.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(AnimationState::Idle)
        .insert(Animation {
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
            frames: (16..24).collect(),
            frame_idx: 0,
        })
        .insert(SpriteFlip(false))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::default())
        .with_children(|parent| {
            parent
                .spawn(Collider::ball(15.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -8.0, 0.0)))
                .insert(Friction::coefficient(0.0));
        });
}

fn draw_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut show_window_res: ResMut<ShowWindow>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    player_q: Query<(&Transform, &Velocity), With<Player>>,
) {
    let mut player_pos = Vec3::default();
    let mut tile_pos = TilePos::default();
    let mut velocity = Vec2::default();
    for (t, v) in player_q.iter() {
        player_pos = t.translation;
        velocity = v.linvel;

        for (map_size, grid_size, _, _, map_transform) in tilemap_q.iter() {
            let player_pos_2map =
                (map_transform.compute_matrix().inverse() * Vec4::from((player_pos, 1.0))).xy();
            tile_pos = TilePos::from_world_pos(
                &player_pos_2map,
                &map_size,
                &grid_size,
                &TilemapType::Square,
            )
            .unwrap();
            break;
        }
    }

    let ctx = egui_ctx.ctx_mut();

    egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "View", |ui| {
                if ui.button("Window").clicked() {
                    show_window_res.0 = !show_window_res.0;
                    ui.close_menu();
                }
            });
        });
    });

    if show_window_res.0 {
        egui::Window::new("[DEBUG]").show(ctx, |ui| {
            ui.label(format!("Player position: {:?}", player_pos));
            ui.label(format!("Player tiled position: {:?}", tile_pos));
            ui.label(format!("Player velocity: {:?}", velocity));
            if ui.button("Close").clicked() {
                show_window_res.0 = false;
            }
        });
    }
}

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
        .add_plugin(TilemapPlugin)
        .add_plugin(tiled::TiledMapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ShowWindow(true))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(VelocityMultiplier(120.0))
        .init_resource::<CursorPos>()
        .init_resource::<TilesProperties>()
        .add_startup_system_to_stage(StartupStage::PreStartup, startup)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(debug::debug_layer)
        .add_system(movement::player_movement.label(PrototypSystemLabel::Movement))
        .add_system(movement::camera_movement)
        .add_system(cursor_pos::update_cursor_pos)
        .add_system(
            animation::update_animation
                .label(PrototypSystemLabel::UpdateAnimation)
                .after(PrototypSystemLabel::Movement),
        )
        .add_system(animation::animate.after(PrototypSystemLabel::UpdateAnimation))
        .add_system(draw_ui)
        .run();
}
