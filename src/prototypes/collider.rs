use bevy::prelude::{AssetServer, Res};
use bevy_rapier2d::prelude::{Collider, Friction, LockedAxes, RigidBody, Velocity, KinematicCharacterController, ActiveEvents, ActiveCollisionTypes};
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum ColliderShape {
    Capsule(f32, f32),
    Circle(f32),
}

#[derive(Clone, Serialize, Deserialize)]
struct ColliderDef(ColliderShape);

#[typetag::serde]
impl ProtoComponent for ColliderDef {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        let collider = match self.0 {
            ColliderShape::Capsule(x, y) => Collider::capsule_y(x, y),
            ColliderShape::Circle(x) => Collider::ball(x / 2.0),
        };

        commands.insert(collider)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC);
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

#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
#[proto_comp(into = "KinematicCharacterController")]
struct RapierController;

impl From<RapierController> for KinematicCharacterController {
    fn from(_: RapierController) -> Self {
        KinematicCharacterController::default()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct PhysicsDefault {
    kind: RigidBody,
}

#[typetag::serde]
impl ProtoComponent for PhysicsDefault {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        commands
            .insert(self.kind)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED);
    }
}
