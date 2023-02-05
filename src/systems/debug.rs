use bevy::{log, prelude::*};

use crate::components::Player;

pub fn debug_layer(
    mut player_q: Query<&mut TextureAtlasSprite, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut inc = 0;
    if keyboard_input.just_pressed(KeyCode::F) {
        inc += 1;
    }

    if keyboard_input.just_pressed(KeyCode::G) {
        inc -= 1;
    }

    if inc != 0 {
        let mut s = player_q.single_mut();
        s.index += inc;
        log::info!("Sprite idx: {}", s.index);
    }
}
