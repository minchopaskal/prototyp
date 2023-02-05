use bevy::prelude::SystemLabel;

mod helpers;

pub mod animation;
pub mod cursor_pos;
pub mod debug;
pub mod movement;

#[derive(SystemLabel)]
pub enum PrototypSystemLabel {
    Movement,
    UpdateAnimation,
}
