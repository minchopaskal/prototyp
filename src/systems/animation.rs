use bevy::{log, prelude::*};

use crate::components::{Animation, AnimationState, SpriteFlip};

pub fn update_animation(
    mut changed_animation_q: Query<(&mut Animation, &AnimationState), Changed<AnimationState>>,
) {
    // TODO: all of this should be from a config
    for (mut animation, state) in changed_animation_q.iter_mut() {
        match state {
            AnimationState::Walking => {
                *animation = Animation {
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                    frames: (56..64).collect(),
                    frame_idx: 0,
                };
            }
            AnimationState::Running => {
                *animation = Animation {
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                    frames: (48..56).collect(),
                    frame_idx: 0,
                };
            }
            _ => {
                // Idle
                assert!(*state == AnimationState::Idle);
                *animation = Animation {
                    timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                    frames: (16..24).collect(),
                    frame_idx: 0,
                };
            }
        }
        log::info!("Changed animation: {:?}", *state);
    }
}

pub fn animate(
    mut anim_q: Query<(&mut Animation, &mut TextureAtlasSprite, Option<&SpriteFlip>)>,
    time: Res<Time>,
) {
    for (mut anim, mut sprite, flip) in anim_q.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.finished() {
            anim.frame_idx = (anim.frame_idx + 1) % anim.frames.len() as u8;
            *sprite = TextureAtlasSprite::new(anim.frames[anim.frame_idx as usize] as usize);
            sprite.flip_x = if flip.is_none() {
                false
            } else {
                flip.unwrap().0
            };
        }
    }
}
