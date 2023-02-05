use bevy::prelude::*;

use crate::resources::CursorPos;
use crate::systems::helpers::window_pos_in_world;

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
