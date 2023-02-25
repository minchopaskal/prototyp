use bevy::{app::AppExit, log, prelude::*, math::Vec4Swizzles};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::Velocity;

use crate::{components::{Player, MainCamera}, resources::{UiSettings, CursorPos}};
use crate::systems::helpers::window_pos_in_world;

pub fn debug_input(
    mut player_q: Query<&mut TextureAtlasSprite, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    let mut inc = 0;
    if keyboard_input.just_pressed(KeyCode::F) {
        inc += 1;
    }

    if keyboard_input.just_pressed(KeyCode::G) {
        inc -= 1;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    if inc != 0 {
        let mut s = player_q.single_mut();
        s.index += inc;
        log::info!("Sprite idx: {}", s.index);
    }
}

pub fn update_cursor_pos(
    windows: Res<Windows>,
    camera_q: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        for (cam_t, cam) in camera_q.iter() {
            *cursor_pos = CursorPos(window_pos_in_world(
                &windows,
                cursor_moved.position,
                cam_t,
                cam,
            ));
        }
    }
}

pub fn draw_debug_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_settings: ResMut<UiSettings>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    player_q: Query<(&Transform, &Velocity), With<Player>>,
    camera_q: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
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
            .unwrap_or(TilePos { x: 9999, y: 9999 });
            break;
        }
    }

    let (cam_t, ortho) = camera_q.single();

    let ctx = egui_ctx.ctx_mut();

    egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "View", |ui| {
                if ui.button("Debug Window").clicked() {
                    ui_settings.show_debug_window = !ui_settings.show_debug_window;
                    ui.close_menu();
                }
            });
        });
    });

    if ui_settings.show_debug_window {
        egui::Window::new("[DEBUG]").show(ctx, |ui| {
            ui.label(format!("Player position: {:?}", player_pos));
            ui.label(format!("Player tiled position: {:?}", tile_pos));
            ui.label(format!("Player velocity: {:?}", velocity));
            ui.label(format!("Camera transform: {:?}", cam_t));
            ui.label(format!("Camera zoom: {:?}", 1.0 / ortho.scale));

            if ui.button("Close").clicked() {
                ui_settings.show_debug_window = false;
            }
        });
    }
}