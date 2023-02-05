use crate::{
    components::{AnimationState, Player, SpriteFlip},
    resources::VelocityMultiplier,
};
use bevy::{
    input::Input,
    math::{Vec3, Vec3Swizzles},
    prelude::*,
    render::camera::Camera,
};
use bevy_rapier2d::prelude::Velocity;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (&mut Velocity, &mut SpriteFlip, &mut AnimationState),
        (With<Player>, Without<Camera>),
    >,
    velocity_mult: Res<VelocityMultiplier>,
) {
    let mut fast = 1.0;
    if keyboard_input.pressed(KeyCode::LShift) {
        fast = 2.0;
    }

    let mut direction = Vec3::ZERO;
    let (velocity_mut, sprite_flip, anim_state) = &mut player_query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
        sprite_flip.0 = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
        sprite_flip.0 = false;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    let mut new_state = AnimationState::Idle;
    if direction != Vec3::ZERO {
        new_state = if fast != 1.0 {
            AnimationState::Running
        } else {
            AnimationState::Walking
        };
        velocity_mut.linvel = direction.xy() * velocity_mult.0 * fast;
    } else {
        velocity_mut.linvel = Vec2::ZERO;
    }

    if *anim_state.as_ref() != new_state {
        *anim_state.as_mut() = new_state;
    }
}

pub fn camera_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut camera_transform, mut ortho) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 2.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 2.0 * time.delta_seconds();
        }

        if ortho.scale < 0.3 {
            ortho.scale = 0.3;
        }

        if ortho.scale > 10.0 {
            ortho.scale = 10.0;
        }

        let player_transform = player_q.single();
        let z = camera_transform.translation.z;

        if (camera_transform.translation.xy() - player_transform.translation.xy()).length() < 1.0 {
            continue;
        }

        camera_transform.translation = player_transform.translation;
        camera_transform.translation.z = z;
    }
}
