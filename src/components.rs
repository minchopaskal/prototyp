use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Default, Serialize, Deserialize, ProtoComponent)]
pub struct Player;

#[derive(Component, Default)]
pub struct Actor;

#[derive(Component, Default)]
pub struct NPC;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Clone, Component, Default, PartialEq, Debug, Serialize, Deserialize, ProtoComponent)]
pub enum AnimationState {
    None,
    #[default]
    Idle,
    Walking,
    Running,
}

#[derive(Clone, Component, Default, Debug, Serialize, Deserialize, ProtoComponent)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<u8>,
    pub frame_idx: u8,
}

#[derive(Clone, Component, Default, Debug, PartialEq, PartialOrd)]
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

#[derive(Bundle, Clone, Default)]
pub struct AnimationBundle {
    animation_state: AnimationState,
    animation: Animation,
    direction: Direction,
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
