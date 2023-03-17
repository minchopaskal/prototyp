use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Default, Serialize, Deserialize, ProtoComponent)]
pub struct Player;

#[derive(Component, Default)]
pub struct Actor;

pub type NpcId = usize;

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize, Component, ProtoComponent)]
pub struct NPC(pub NpcId);


#[derive(Default, Clone)]
#[derive(Serialize, Deserialize, Component, ProtoComponent)]
pub struct AI;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
#[derive(Serialize, Deserialize, Component, ProtoComponent)]
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AnimationDef {
    pub frame_cnt: u8,
    pub first_frame_idx: u8,
    pub dir_offset: u8,
    pub fps: f32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[derive(Component)]
pub struct EntityAnimationData {
    pub animations: std::collections::HashMap<AnimationState, AnimationDef>
}

#[derive(Bundle, Clone, Default)]
pub struct AnimationBundle {
    pub animation_state: AnimationState,
    pub animation: Animation,
    pub direction: Direction,
    pub animations: EntityAnimationData,
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
