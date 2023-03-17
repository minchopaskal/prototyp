use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;
use relative_path::RelativePath;
use std::fs::{self};

use crate::components::MainCamera;
use crate::prototypes::spawn_prototype;
use crate::tiled;

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
    proto_data: Res<ProtoData>,
) {
    spawn_prototype("player", &mut commands, &asset_server, &proto_data)
}
