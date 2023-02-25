use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::tiled;
use crate::components::{MainCamera, Player, AnimationState, Animation, Direction};

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<tiled::TiledMap> = asset_server.load("map/simple.tmx");

    commands.spawn(tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlasses: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load("character/doctor/doctor.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.0), 4, 18, None, None);
    let atlas_handle = atlasses.add(atlas);

    let sprite = TextureAtlasSprite::new(0);

    commands
        .spawn(Player)
        .insert(Name::new("Player"))
        .insert(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas_handle.clone(),
            ..Default::default()
        })
        .insert(Transform::from_xyz(20.0, 0.0, 10.0))
        .insert(AnimationState::Idle)
        .insert(Animation {
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
            frames: (0..1).collect(),
            frame_idx: 0,
        })
        .insert(Direction::default())
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::default())
        .with_children(|parent| {
            parent
                .spawn(Collider::capsule_y(10.0, 7.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
                .insert(Friction::coefficient(0.0));
        });
}
