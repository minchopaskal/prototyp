use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;

use crate::{resources::{NpcPool, NpcData}, prototypes::spawn_prototype};

pub fn spawn_npcs(mut commands: Commands, mut npc_res: ResMut<NpcPool>, asset_server: Res<AssetServer>, proto_data: Res<ProtoData>) {
    if !npc_res.is_changed() {
        return;
    }

    println!("Npcs: {:?}", npc_res.npcs);

    for NpcData{ name, pos } in npc_res.npcs.iter() {
        let id = spawn_prototype(&name, &mut commands, &asset_server, &proto_data);
        commands.entity(id).insert(SpatialBundle::from_transform(Transform::from_xyz(pos.x, pos.y, pos.z)));
    }

    npc_res.npcs.clear()
}
