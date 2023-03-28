use bevy::prelude::*;

use crate::{components::{InNpcReach, HintEntityWrapper, Player, DialogueEntityWrapper, Empty}, dialogue::{Dialogue, DialogueTree}};

use super::text;

pub fn resolve_dialogue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<(Entity, &InNpcReach), With<Player>>,
    hint_q: Query<&HintEntityWrapper>,
    mut dialogue_q: Query<&mut Dialogue>,
    text_q: Query<&DialogueEntityWrapper>,
) {
    if let Ok((entt, diag_with)) = player_q.get_single() {
        let player = entt;
        let npc = diag_with.0;
        
        if keyboard_input.just_pressed(KeyCode::E) {
            if let Ok(HintEntityWrapper(entt)) = hint_q.get(player) {
                commands.entity(*entt).despawn_recursive();
                commands.entity(player).remove::<HintEntityWrapper>();
            }

            if let Ok(text_entt) = text_q.get(player) {
                commands.entity(text_entt.0).despawn_recursive();
                commands.entity(player).remove::<DialogueEntityWrapper>();
            }

            if let Ok(mut diag) = dialogue_q.get_mut(npc) {
                if diag.curr_exchange >= diag.exchanges.len(){
                    return;
                }
                
                if let DialogueTree::List(lines) = &diag.exchanges[diag.curr_exchange] {
                    if diag.curr_line >= lines.len() {
                        return;
                    }

                    let line = &lines[diag.curr_line];
                    let name = &diag.participants[line.author].name;

                    let diag_entt = text::spawn_dialog_box::<Empty>(&mut commands, &asset_server, &format!("{name}"), &line.text, None);

                    let diag_entt = DialogueEntityWrapper(diag_entt);
                    commands.entity(player)
                        .insert(diag_entt);
                }

                diag.curr_line += 1;
            }
        }
    }
}