use bevy::prelude::SystemLabel;

mod helpers;

pub mod animation;
pub mod collision;
pub mod debug;
pub mod movement;
pub mod setup;
pub mod sign;
pub mod text;
pub mod npc;

#[derive(SystemLabel)]
pub enum PrototypSystemLabel {
    Movement,
    UpdateAnimation,
    SignUpdate,
}
