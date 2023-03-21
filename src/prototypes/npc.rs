use bevy::{prelude::Component, reflect::Reflect};
use bevy_proto::prelude::ProtoComponent;
use serde::{Serialize, Deserialize};

#[derive(Component, ProtoComponent, Serialize, Deserialize, Clone, Reflect)]
pub struct Speed(pub f32);