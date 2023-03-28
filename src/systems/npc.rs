use std::path::PathBuf;

use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;
use bevy_rapier2d::prelude::ActiveHooks;

use crate::{resources::{NpcPool, NpcData}, prototypes::spawn_prototype, components::{AI, NPC, AIKind}, dialogue::Dialogue};

use super::collision::PhysicsFilterTag;

pub fn spawn_npcs(
    mut commands: Commands,
    mut npc_res: ResMut<NpcPool>,
    asset_server: Res<AssetServer>,
    proto_data: Res<ProtoData>
) {
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

pub fn spawn_npc_dialogues(
    mut commands: Commands,
    ai_q: Query<(Entity, &NPC, &AI), Without<Dialogue>>,
) {
    for (entt, npc, ai) in ai_q.iter() {
        if ai.kind != AIKind::Talking {
            continue;
        }

        let mut filename = npc.0.to_string();
        filename.push_str(".diag");

        let file = PathBuf::from("assets").join("dialogues").join(filename);

        log::info!("Loading dialogue {:?} for npc {}", file, npc.0);

        let diag = Dialogue::new(file);
        commands.entity(entt).insert(diag);
    }
}
