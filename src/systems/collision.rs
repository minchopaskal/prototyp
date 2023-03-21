use bevy::prelude::*;
use bevy_rapier2d::{prelude::{CollisionEvent, Collider}, rapier::prelude::CollisionEventFlags};

use crate::{components::{Player, NPC, NPCDialogMarker, TextEntityWrapper}, systems::text};

pub fn handle_player_npc_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    player_q: Query<Entity, With<Player>>,
    npc_q: Query<&NPC>,
    dialog_q: Query<&TextEntityWrapper>,
    collider_q: Query<&Parent, With<Collider>>,
) {
    let player = player_q.get_single().unwrap();

    for collision_event in collision_events.iter() {
        match *collision_event {
        CollisionEvent::Started(e1, e2, flags) => {
            if flags & CollisionEventFlags::REMOVED == CollisionEventFlags::REMOVED {
                continue;
            }

            let mut player_collider = Entity::from_raw(0);
            let mut npc = Entity::from_raw(0);

            if let Ok(parent) = collider_q.get(e1) {
                if parent.get() == player {
                    player_collider = e1;
                }
                if let Ok(_) = npc_q.get(parent.get()) {
                    npc = parent.get();
                }
            }
            if let Ok(parent) = collider_q.get(e2) {
                if parent.get() == player {
                    player_collider = e2;
                }
                if let Ok(_) = npc_q.get(parent.get()) {
                    npc = parent.get();
                }
            }

            if player_collider.index() != 0 && npc.index() != 0 {
                if let Ok(_) = dialog_q.get(player_collider) {
                    continue;
                }

                let npc_id = npc_q.get(npc).unwrap().0;

                let diag_entt = text::spawn_dialog_box(&mut commands, &asset_server, &format!("NPC[{npc_id}]"), "You got me!", Some(NPCDialogMarker));

                let diag_entt = TextEntityWrapper(diag_entt);
                commands.entity(player_collider).insert(diag_entt);
            }
        },
        CollisionEvent::Stopped(e1, e2, _) => {
            if let Ok(TextEntityWrapper(id)) = dialog_q.get(e1) {
                commands.entity(*id).despawn_recursive();
                commands.entity(e1).remove::<TextEntityWrapper>();
            }
            if let Ok(TextEntityWrapper(id)) = dialog_q.get(e2) {
                commands.entity(*id).despawn_recursive();
                commands.entity(e2).remove::<TextEntityWrapper>();
            }
        },
        }
    }
}