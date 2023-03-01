use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
#[proto_comp(into = "Name")]
struct NameDef(String);

impl From<NameDef> for Name {
    fn from(name: NameDef) -> Self {
        Name::new(name.0)
    }
}

#[derive(Clone, Serialize, Deserialize, Component)]
struct TransformDef(f32, f32, f32);

#[typetag::serde]
impl ProtoComponent for TransformDef {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        commands.insert(SpatialBundle::from(Transform::from_xyz(
            self.0, self.1, self.2,
        )));
    }
}
