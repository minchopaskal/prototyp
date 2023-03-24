use bevy::prelude::{AssetServer, Res};
use bevy_rapier2d::prelude::{Collider, Friction, LockedAxes, RigidBody, Velocity, KinematicCharacterController, ActiveEvents, ActiveCollisionTypes, AdditionalMassProperties, ActiveHooks};
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

use crate::systems::collision::PhysicsFilterTag;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum ColliderShape {
    Capsule(f32, f32),
    Circle(f32),
}

#[derive(Clone, Serialize, Deserialize)]
struct ColliderDef {
    shape: ColliderShape,
    collision_events: Option<bool>,
}

#[typetag::serde]
impl ProtoComponent for ColliderDef {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        let collider = match self.shape {
            ColliderShape::Capsule(x, y) => Collider::capsule_y(x, y),
            ColliderShape::Circle(x) => Collider::ball(x / 2.0),
        };

        commands.insert(collider);

        if let Some(e) = self.collision_events {
            if e {
                commands.insert(ActiveEvents::COLLISION_EVENTS);
            }
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
    mass: Option<f32>,
}

#[typetag::serde]
impl ProtoComponent for PhysicsDefault {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        let mass = AdditionalMassProperties::Mass(
            if self.mass.is_some() {
                self.mass.unwrap()
            } else {
                1000.0
            }
        );
        commands
            .insert(self.kind)
            .insert(mass)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED);
    }
}
