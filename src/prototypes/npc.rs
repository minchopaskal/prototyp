use bevy::{prelude::{Component, AssetServer, Res, BuildChildren, SpatialBundle}, reflect::Reflect};
use bevy_proto::prelude::{ProtoComponent, ProtoCommands};
use bevy_rapier2d::prelude::{Collider, Sensor, ActiveEvents};
use serde::{Serialize, Deserialize};

use crate::{components::{AI, AIKind}, dialogue::Dialogue};

#[derive(Component, ProtoComponent, Serialize, Deserialize, Clone, Reflect)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(100.0)
    }
}

#[typetag::serde]
impl ProtoComponent for AI {
    fn insert_self(&self, commands: &mut ProtoCommands, _: &Res<AssetServer>) {
        match self.kind {
        AIKind::None => { unreachable!() },
        AIKind::RunAway => {
            commands.insert(AI{ kind: AIKind::RunAway});
        },
        AIKind::Talking => {
            commands
                .insert(AI{ kind: AIKind::Talking})
                .with_children(|parent| {
                    parent
                        .spawn(Collider::ball(32.0))
                        .insert(Sensor)
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(SpatialBundle::default());
            });
        },    
        }
    }
}