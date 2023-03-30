use bevy::prelude::*;

use crate::{components::{InNpcReach, HintEntityWrapper, Player, DialogueEntityWrapper, Empty, NPC}, dialogue::{Dialogue, DialogueTree}, resources::VariablePool};

use super::text;

// TODO: resolve variables/names/etc...
pub fn resolve_text(
    text: &str,
    suffix: Option<&str>,
    player_q: &Query<&Name, With<Player>>,
    npc_name_q: &Query<(&Name, &NPC), Without<Player>>,
    _variables: &Res<VariablePool>,
) -> String {
    let mut res = String::new();

    let mut fst = true;
    let mut split = text.split(" ").peekable();
    while let Some(word) = split.next() {
        if !fst && split.peek().is_some() {
            res.push(' ');
        }
        fst = false;

        if !word.starts_with('_') {
            res.push_str(word);
            continue;
        }

        let mut trimmed = word.trim_end_matches([',', '.',]).to_string();

        if let Some(suffix) = suffix {
            trimmed.push_str(suffix);
        }

        // TODO: resolve variables
        if trimmed == "_player_name" {
            let name = player_q.get_single().unwrap_or(&Name::default()).as_str().to_string();
            res.push_str(&name);
        }

        if trimmed.starts_with("_npc_"){
            let mut npc = trimmed.split('_');
            
            // skip "" and "npc"
            npc.next();
            npc.next();

            let npc_id = npc.next();
            let npc_id = npc_id.unwrap_or("-1").parse::<i32>().unwrap_or(-1);

            let var = npc.next().unwrap_or("");

            if npc_id >= 0 && var == "name" {
                for npc in npc_name_q.iter() {
                    if npc.1.0 == npc_id as usize {
                        res.push_str(npc.0.as_str());
                        break;
                    }
                }
            }
        }

        if trimmed.len() < word.len() {
            res.push_str(&word[trimmed.len()..]);
        }
    }

    res
}

pub fn resolve_dialogue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<(Entity, &InNpcReach), With<Player>>,
    hint_q: Query<&HintEntityWrapper>,
    mut dialogue_q: Query<&mut Dialogue>,
    text_q: Query<&DialogueEntityWrapper>,
    player_name_q: Query<&Name, With<Player>>,
    npc_name_q: Query<(&Name, &NPC), Without<Player>>,
    variables: Res<VariablePool>,
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
                    let name = resolve_text(&diag.participants[line.author].name, Some(&"_name"), &player_name_q, &npc_name_q, &variables);
                    let text = resolve_text(&line.text, None, &player_name_q, &npc_name_q, &variables);

                    let diag_entt = text::spawn_dialog_box::<Empty>(&mut commands, &asset_server, &format!("{name}"), &text, None);

                    let diag_entt = DialogueEntityWrapper(diag_entt);
                    commands.entity(player)
                        .insert(diag_entt);
                }

                diag.curr_line += 1;
            }
        }
    }
}