use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{EntityPair, DialogueEntityWrapper, MainCamera, Sign, SignTextMarker, Player},
    resources::SignsPool,
    systems::text::{self, TextPosition, TextValue, TextBuilder},
};

pub fn add_sign_sensors(mut commands: Commands, signs_res: Res<SignsPool>) {
    if !signs_res.is_changed() {
        return;
    }

    for (handle, sign) in signs_res.signs.iter().enumerate() {
        commands
            .spawn(Sign { handle })
            .insert(TransformBundle::from(Transform::from_xyz(
                sign.x, sign.y, 10.0,
            )))
            .with_children(|parent| {
                parent
                    .spawn(Collider::ball(16.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(TransformBundle::from(Transform::default()));
            });
    }
}

// Use below two to show text above some object.
// We'll need to abstract objects in some way.
// TODO: There should be only one method handling collisions.
pub fn handle_sign_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    sensor_q: Query<&Parent, With<Sensor>>,
    parents_q: Query<&Parent>,
    sign_q: Query<&Sign>,
    player_q: Query<Entity, With<Player>>,
    entt_pairs_q: Query<(&EntityPair, &DialogueEntityWrapper)>,
    camera_q: Query<(&Camera, &GlobalTransform, &OrthographicProjection), With<MainCamera>>,
    signs_res: Res<SignsPool>,
    windows: Res<Windows>,
) {
    enum SignId {
        Start(usize, Entity, Entity),
        Stop(Entity, Entity),

        Invalid,
    }

    let window = windows.primary();

    let player_entt;
    if let Ok(p) = player_q.get_single() {
        player_entt = p;
    } else {
        return;
    }

    for collision_event in collision_events.iter() {

        let sign_id = match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let (sign_entt, mut other_entt) = if let Ok(p) = sensor_q.get(*e1) {
                    (p.get(), *e2)
                } else if let Ok(p) = sensor_q.get(*e2) {
                    (p.get(), *e1)
                } else {
                    (Entity::from_raw(0), Entity::from_raw(0))
                };

                if let Ok(p) = parents_q.get(other_entt) {
                    other_entt = p.get();
                }

                if other_entt == player_entt {
                    if let Ok(sign) = sign_q.get(sign_entt) {
                        SignId::Start(sign.handle, *e1, *e2)
                    } else {
                        SignId::Invalid
                    }
                } else {
                    SignId::Invalid
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => SignId::Stop(*e1, *e2),
        };

        match sign_id {
            SignId::Start(handle, e1, e2) => {
                let sign_data = &signs_res.signs[handle];
                let id = sign_data.id;

                let (camera, camera_transform, ortho) = camera_q.single();

                // TODO: see why world_to_viewport doesn't work
                let world_pos = Vec3 {
                    x: sign_data.x,
                    y: sign_data.y,
                    z: 100.0,
                };
                let ndc = camera.world_to_ndc(camera_transform, world_pos).unwrap();
                let ndc = (ndc.truncate() + Vec2::ONE) / 2.0;
                let perceived_tile_size = 32.0 / ortho.scale + 16.0;
                let pos = Vec2 {
                    x: window.width() * ndc.x,
                    // add offset so text appears above sign.
                    // TODO: if direction is DOWN show on the bottom. Also take note of camera zoom
                    y: window.height() - window.height() * ndc.y - perceived_tile_size,
                };

                let text = format!("Reading sign id: {}", id);
                let sign_text_entt = text::spawn_text(
                    TextBuilder::Commands(&mut commands),
                    &asset_server,
                    vec![TextValue::Dialogue(&text)],
                    TextPosition::Absolute(pos.x, pos.y),
                    false,
                    Some(SignTextMarker),
                );

                commands
                    .spawn(EntityPair(e1, e2))
                    .insert(DialogueEntityWrapper(sign_text_entt));
            }
            SignId::Stop(e1, e2) => {
                for pair in entt_pairs_q.iter() {
                    if pair.1.0.index() == 0 {
                        break;
                    }

                    if *pair.0 == EntityPair(e1, e2) || *pair.0 == EntityPair(e2, e1) {
                        if let Some(entt) = commands.get_entity(pair.1 .0) {
                            entt.despawn_recursive();
                        }
                    }
                }
            }
            _ => (),
        };
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

pub fn fix_sign_style(
    mut commands: Commands,
    mut style_q: Query<(Entity, &mut Style, &Node), Added<SignTextMarker>>,
) {
    for (entt, mut style, node) in style_q.iter_mut() {
        let offset = node.size().x / 2.0;
        if let Val::Px(curr_left) = style.position.left {
            let curr_pos = style.position;

            *style = Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: curr_pos.top,
                    left: Val::Px(curr_left - offset),
                    ..Default::default()
                },
                ..Default::default()
            };
        }

        commands
            .entity(entt)
            .insert(Visibility { is_visible: true });
    }
}
