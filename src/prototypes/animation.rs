use bevy::prelude::*;
use bevy_proto::prelude::*;

use crate::components::{AnimationBundle, EntityAnimationData};

#[typetag::serde]
impl ProtoComponent for EntityAnimationData {
    fn insert_self(&self, commands: &mut ProtoCommands, _: &Res<AssetServer>) {
        let animations = self.clone();
        commands.insert(AnimationBundle {
            animations,
            ..default()
        });
    }
}