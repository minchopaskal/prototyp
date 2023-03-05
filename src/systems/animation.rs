use bevy::prelude::*;

use crate::components::{Animation, AnimationState, Direction, EntityAnimationData};

pub fn update_character_animation(
    mut changed_animation_q: Query<
        (&mut Animation, &AnimationState, &Direction, &EntityAnimationData),
        Or<(Changed<AnimationState>, Changed<Direction>)>,
    >,
) {
    // TODO: all of this should be from a config
    for (mut animation, state, dir, anim_data) in changed_animation_q.iter_mut() {
        let anim_data = &anim_data.animations[state];
        let mut offset = (anim_data.first_frame_idx, anim_data.frame_cnt);

        offset.0 += match dir {
            Direction::Down => 0,
            Direction::DownRight => anim_data.dir_offset,
            Direction::Right => 2 * anim_data.dir_offset,
            Direction::UpRight => 3 * anim_data.dir_offset,
            Direction::Up => 4 * anim_data.dir_offset,
            Direction::UpLeft => 5 * anim_data.dir_offset,
            Direction::Left => 6 * anim_data.dir_offset,
            Direction::DownLeft => 7 * anim_data.dir_offset,
        };

        *animation = Animation {
            timer: Timer::from_seconds(1.0 / anim_data.fps, TimerMode::Repeating),
            frames: (offset.0..offset.0 + offset.1).collect(),
            frame_idx: 0,
        };
    }
}

pub fn animate(mut anim_q: Query<(&mut Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut anim, mut sprite) in anim_q.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.finished() {
            anim.frame_idx = (anim.frame_idx + 1) % anim.frames.len() as u8;
            *sprite = TextureAtlasSprite::new(anim.frames[anim.frame_idx as usize] as usize);
        }
    }
}
