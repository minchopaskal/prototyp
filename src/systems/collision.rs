use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use serde::{Serialize, Deserialize};

use crate::{components::{Player, NPC, NPCDialogMarker, DialogueEntityWrapper, AI, AIKind, InNpcReach, HintEntityWrapper, Empty}, systems::text};

use super::text::{spawn_text, TextValue};

#[derive(Component, Copy, Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize, Reflect)]
pub enum PhysicsFilterTag {
    Player,
    Npc,
}

pub struct PlayerNpcContantFilter;
impl PhysicsHooksWithQuery<&PhysicsFilterTag> for PlayerNpcContantFilter {
    fn filter_contact_pair(
        &self,
        context: PairFilterContextView,
        tag_q: &Query<&PhysicsFilterTag>,
    ) -> Option<SolverFlags> {
        if let Ok(tag1) = tag_q.get(context.collider1()) {
            if let Ok(tag2) = tag_q.get(context.collider2()) {
                return Some(SolverFlags::empty())
            }
        }
        Some(SolverFlags::COMPUTE_IMPULSES)
    }
}

pub fn handle_player_npc_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    player_q: Query<Entity, With<Player>>,
    npc_q: Query<(&NPC, &AI)>,
    parent_q: Query<&Parent>,
    dialog_q: Query<&DialogueEntityWrapper>,
    in_reach_q: Query<&InNpcReach>,
    hint_q: Query<&HintEntityWrapper>,
) {
    let player = player_q.get_single().unwrap();

    for collision_event in collision_events.iter() {
        match *collision_event {
        CollisionEvent::Started(e1, e2, flags) => {
            if flags & CollisionEventFlags::REMOVED == CollisionEventFlags::REMOVED {
                continue;
            }

            let mut npc = None;
            let mut is_player = false;

            for e in [e1, e2] {
                // Check if collider belongs to player
                if e == player {
                    is_player = true;
                }

                // Check if collider belongs to npc
                if let Ok(entt) = npc_q.get(e) {
                    npc = Some((entt.0, entt.1, e));
                }

                // Check if collider is child of npc (f.e sensor)
                if let Ok(p) = parent_q.get(e) {
                    if let Ok(entt) = npc_q.get(p.get()) {
                        npc = Some((entt.0, entt.1, p.get()));
                    }
                }
            }

            if is_player && npc.is_some(){
                if let Ok(_) = dialog_q.get(player) {
                    continue;
                }
                let entt = npc.unwrap();

                let npc_id = entt.0.0;
                match entt.1.kind {
                AIKind::None => unreachable!(),
                AIKind::RunAway => {
                    let diag_entt = text::spawn_dialog_box(&mut commands, &asset_server, &format!("NPC [{npc_id}]"), "You got me!", Some(NPCDialogMarker));

                    let diag_entt = DialogueEntityWrapper(diag_entt);
                    commands.entity(player).insert(diag_entt);
                },
                AIKind::Talking => {
                    commands.entity(player).insert(InNpcReach(entt.2));
                    if !hint_q.contains(player) {
                        let hint_entt = spawn_text::<Empty>(
                            &mut commands,
                            &asset_server,
                            vec![TextValue::Dialogue(&"Press E to talk")],
                            text::TextPosition::Percent(40, 90),
                            true,
                            None,
                        );
                        commands.entity(player).insert(HintEntityWrapper(hint_entt));
                    }
                },
                }
            }
        },
        CollisionEvent::Stopped(e1, e2, _) => {
            for e in [e1, e2] {
                if let Ok(DialogueEntityWrapper(id)) = dialog_q.get(e) {
                    commands.entity(*id).despawn_recursive();
                    commands.entity(e).remove::<DialogueEntityWrapper>();
                }

                if let Ok(InNpcReach(_)) = in_reach_q.get(e) {
                    commands.entity(e).remove::<InNpcReach>();
                    commands.entity(e).remove::<HintEntityWrapper>();
                }

                if let Ok(HintEntityWrapper(entt)) = hint_q.get(e) {
                    commands.entity(*entt).despawn_recursive();
                } 
            }
        },
        }
    }
}

pub fn check_npc_in_reach(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_q: Query<(Entity, &InNpcReach)>,
    text_q: Query<&DialogueEntityWrapper>,
    hint_q: Query<&HintEntityWrapper>,
    npc_q: Query<&NPC>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(entt) = player_q.get_single() {
        let player = entt.0;

        if let Ok(text_entt) = text_q.get(entt.0) {
            if keyboard_input.just_pressed(KeyCode::E) {
                commands.entity(text_entt.0).despawn_recursive();
                commands.entity(player).remove::<DialogueEntityWrapper>();
                return;
            }
        }

        if let Ok(npc) = npc_q.get(entt.1.0) {
            let npc_id = npc.0;
            if keyboard_input.just_pressed(KeyCode::E) {

                let diag_entt = text::spawn_dialog_box(&mut commands, &asset_server, &format!("NPC [{npc_id}]"), "You got me!", Some(NPCDialogMarker));

                if let Ok(HintEntityWrapper(entt)) = hint_q.get(player) {
                    commands.entity(*entt).despawn_recursive();
                    commands.entity(player).remove::<HintEntityWrapper>();
                } 

                let diag_entt = DialogueEntityWrapper(diag_entt);
                commands.entity(player)
                    .insert(diag_entt);
            }
        }
    }
}