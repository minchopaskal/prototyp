use bevy::prelude::{AssetServer, Res};
use bevy_rapier2d::prelude::{Collider, Friction, LockedAxes, RigidBody, Velocity};
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum ColliderShape {
    Capsule(f32, f32),
    Circle(f32),
}

#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
#[proto_comp(into = "Collider")]
struct ColliderDef(ColliderShape);

impl From<ColliderDef> for Collider {
    fn from(collider: ColliderDef) -> Self {
        match collider.0 {
            ColliderShape::Capsule(x, y) => Collider::capsule_y(x, y),
            ColliderShape::Circle(x) => Collider::ball(x / 2.0),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
#[proto_comp(into = "Friction")]
struct FrictionDef {
    c: f32,
}

impl From<FrictionDef> for Friction {
    fn from(friction: FrictionDef) -> Self {
        Friction::coefficient(friction.c)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct PhysicsDefault;

#[typetag::serde]
impl ProtoComponent for PhysicsDefault {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        commands
            .insert(Velocity::default())
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED);
    }
}
