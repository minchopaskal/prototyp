use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;
use bevy_rapier2d::prelude::ActiveHooks;

use crate::{resources::{NpcPool, NpcData}, prototypes::spawn_prototype};

use super::collision::PhysicsFilterTag;

pub fn spawn_npcs(mut commands: Commands, mut npc_res: ResMut<NpcPool>, asset_server: Res<AssetServer>, proto_data: Res<ProtoData>) {
    if !npc_res.is_changed() {
        return;
    }

    for NpcData{ name, pos } in npc_res.npcs.iter() {
        let id = spawn_prototype(&name, &mut commands, &asset_server, &proto_data);
        commands.entity(id)
            .insert(SpatialBundle::from_transform(Transform::from_xyz(pos.x, pos.y, pos.z)))
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(PhysicsFilterTag::Npc);
    }

    npc_res.npcs.clear()
}
