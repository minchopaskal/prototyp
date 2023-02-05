use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, PartialEq, Debug)]
pub struct TileIdentifier {
    pub tile_id: u16,
    pub tileset_id: u8,
}

#[derive(Component, Default)]
pub struct Actor;

#[derive(Component, Default)]
pub struct NPC;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default, PartialEq, Debug)]
pub enum AnimationState {
    #[default]
    None,
    Idle,
    Walking,
    Running,
}

#[derive(Component, Default, Debug)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<u8>,
    pub frame_idx: u8,
}

// TODO: Actually dir
#[derive(Component, Default)]
pub struct SpriteFlip(pub bool);
