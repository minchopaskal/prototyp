use crate::{
    components::{AnimationState, Direction, Player, AI, NPC, AIKind},
    prototypes::npc::Speed,
};
use bevy::{
    input::Input,
    math::{Vec3, Vec3Swizzles},
    prelude::*,
    render::camera::Camera,
};
use bevy_rapier2d::prelude::{Velocity, KinematicCharacterControllerOutput};

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut AnimationState, &mut Direction, &Speed), With<Player>>,
) {
    let mut fast = 1.0;
    if keyboard_input.pressed(KeyCode::LShift) {
        fast = 2.0;
    }

    let mut view_dir = (0, 0);
    let mut direction = Vec3::ZERO;
    if let Ok((velocity_mut, anim_state, player_dir, speed)) = &mut player_query.get_single_mut() {
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

            velocity_mut.linvel = direction.xy() * speed.0 * fast;
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

fn quantize_dir(dir: Vec2) -> Direction {
    let dir = dir.normalize().extend(0.0);

    let cos_x = dir.dot(Vec3::new(1.0, 0.0, 0.0));
    let cos_y = dir.dot(Vec3::new(0.0, 1.0, 0.0));
    let angle = cos_x.acos() * 180.0 / std::f32::consts::PI;

    if cos_y > 0.0 {
        if angle < 22.5 {
            Direction::Right
        } else if angle < 67.5 {
            Direction::UpRight
        } else if angle < 112.5 {
            Direction::Up
        }  else if angle < 157.5 {
            Direction::UpLeft
        } else {
            Direction::Left
        }
    } else {
        if angle < 22.5 {
            Direction::Right
        } else if angle < 67.5 {
            Direction::DownRight
        } else if angle < 112.5 {
            Direction::Down
        }  else if angle < 157.5 {
            Direction::DownLeft
        } else {
            Direction::Left
        }
    }
}

pub fn ai_movement(
    mut npc_q: Query<(Entity, &mut Velocity, &mut AnimationState, &mut Direction, &mut Transform, &AI), With<NPC>>,
    speed_q: Query<&Speed>,
    player_q: Query<&Transform, (With<Player>, Without<NPC>)>
) {
    let player_pos = player_q.single().translation;

    for (entity, mut velocity, mut state, mut direction, t, ai) in npc_q.iter_mut() {
        let mut new_state = AnimationState::Idle;
        let mut new_dir = *direction.as_ref();
        
        let dir_to_player = t.translation - player_pos;

        match ai.kind {
        AIKind::Talking => {
            if dir_to_player.length() < 128.0 {
                new_dir = quantize_dir(-dir_to_player.xy());
            } else {
                new_dir = Direction::Down;
            }
        },
        AIKind::RunAway => {
            if dir_to_player.length() < 128.0 {
                let mult = 2.0 - dir_to_player.length() / 64.0;
                let default_speed = Speed::default();
                let speed = speed_q.get(entity).unwrap_or(&default_speed);

                velocity.linvel = (mult * speed.0 * dir_to_player.normalize()).xy();
    
                new_state = if mult > 1.0 {
                    AnimationState::Running
                } else if mult > 0.01 {
                    AnimationState::Walking
                } else {
                    AnimationState::Idle
                };
    
                new_dir = quantize_dir(velocity.linvel);
            } else {
                velocity.linvel = Vec2::ZERO;
            }
        },
        _ => { unreachable!() }
        }
        
        // We need to check first, as animation system operates on
        // `Changed` events. If we use .as_mut() changed events are triggered,
        // even though we may not have changed anything.
        if *state.as_ref() != new_state {
            *state = new_state;
        }

        if *direction.as_ref() != new_dir {
            *direction = new_dir;
        }
    }
}