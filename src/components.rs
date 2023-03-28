use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

#[derive(Component, Default)]
pub struct Empty;

#[derive(Clone, Component, Default, Serialize, Deserialize, ProtoComponent)]
pub struct Player;

#[derive(Component, Default)]
pub struct Actor;

pub type NpcId = usize;

#[derive(Default, Clone, Debug)]
#[derive(Serialize, Deserialize, Component, ProtoComponent)]
pub struct NPC(pub NpcId);

#[derive(Default, Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum AIKind {
    #[default]
    None,

    RunAway,
    Talking,
}

#[derive(Default, Clone, Debug)]
#[derive(Serialize, Deserialize, Component)]
pub struct AI {
    pub kind: AIKind,
}

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
    pub first_frame: bool,
}

#[derive(Clone, Copy, Component, Default, Debug, PartialEq, PartialOrd)]
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

#[derive(Component, Reflect)]
pub struct DialogueEntityWrapper(pub Entity); // dialogue box entt

#[derive(Component, Reflect)]
pub struct InDialogueWith(pub Entity); // Npc

#[derive(Component, Reflect)]
pub struct HintEntityWrapper(pub Entity);

#[derive(Component, PartialEq)]
pub struct EntityPair(pub Entity, pub Entity);

#[derive(Component)]
pub struct Sign {
    pub handle: usize,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct NPCDialogMarker;

#[derive(Component)]
pub struct InNpcReach(pub Entity);

impl Default for InNpcReach {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}
