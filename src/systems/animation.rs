use bevy::prelude::*;

use crate::components::{Animation, AnimationState, Direction};

pub fn update_character_animation(
    mut changed_animation_q: Query<
        (&mut Animation, &AnimationState, &Direction),
        Or<(Changed<AnimationState>, Changed<Direction>)>,
    >,
) {
    // TODO: all of this should be from a config
    for (mut animation, state, dir) in changed_animation_q.iter_mut() {
        let mut offset = match state {
            AnimationState::Running => (36, 4),
            AnimationState::Walking => (0, 4),
            AnimationState::Idle => (0, 1),
            _ => (0, 1),
        };

        offset.0 += match dir {
            Direction::Down => 0,
            Direction::DownRight => 4,
            Direction::Right => 8,
            Direction::UpRight => 12,
            Direction::Up => 16,
            Direction::UpLeft => 20,
            Direction::Left => 24,
            Direction::DownLeft => 28,
        };

        *animation = Animation {
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
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
