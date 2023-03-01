use crate::{
    components::{AnimationState, Direction, Player},
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
        (&mut Velocity, &mut AnimationState, &mut Direction),
        (With<Player>, Without<Camera>),
    >,
    velocity_mult: Res<VelocityMultiplier>,
) {
    let mut fast = 1.0;
    if keyboard_input.pressed(KeyCode::LShift) {
        fast = 2.0;
    }

    let mut view_dir = (0, 0);
    let mut direction = Vec3::ZERO;
    if let Ok((velocity_mut, anim_state, player_dir)) = &mut player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            view_dir.0 -= 1;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            view_dir.0 += 1;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            view_dir.1 += 1;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            view_dir.1 -= 1;
        }

        if view_dir != (0, 0) {
            let new_dir = match view_dir {
                (0, -1) => Direction::Down,
                (1, -1) => Direction::DownRight,
                (1, 0) => Direction::Right,
                (1, 1) => Direction::UpRight,
                (0, 1) => Direction::Up,
                (-1, 1) => Direction::UpLeft,
                (-1, 0) => Direction::Left,
                (-1, -1) => Direction::DownLeft,
                _ => Direction::Down,
            };

            if *player_dir.as_ref() != new_dir {
                *player_dir.as_mut() = new_dir;
            }
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
}

pub fn camera_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut camera_transform, mut ortho) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= time.delta_seconds();
        }

        if ortho.scale < 0.3 {
            ortho.scale = 0.3;
        }

        if ortho.scale > 3.0 {
            ortho.scale = 3.0;
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
