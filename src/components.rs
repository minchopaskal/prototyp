use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

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

#[derive(Component, Default, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    #[default]
    Down,
    DownRight,
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
}

#[derive(Component, Default)]
pub struct FPSTextMarker;

#[derive(Component, Default)]
pub struct SignTextMarker;

#[derive(Component)]
pub struct EntityWrapper(pub Entity);

#[derive(Component, PartialEq)]
pub struct EntityPair(pub Entity, pub Entity);

#[derive(Component)]
pub struct Sign {
    pub handle: usize,
}

#[derive(Component)]
pub struct MainCamera;
