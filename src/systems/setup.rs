use bevy::prelude::*;
use bevy_proto::prelude::ProtoData;
use relative_path::RelativePath;
use std::fs::{self};

use crate::components::MainCamera;
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
    let player_proto = proto_data
        .get_prototype("player")
        .expect("Expected player prototype!");
    let player_id = player_proto
        .spawn(&mut commands, &proto_data, &asset_server)
        .id();

    let proto_path = RelativePath::new("assets/prototypes");
    let paths = fs::read_dir(proto_path.to_path("."))
        .expect(&format!("Path {:?} not found!", proto_path.to_path(".")));

    for path in paths {
        if let Err(_) = path {
            continue;
        }

        let path = path.unwrap().file_name();
        let path = path.to_str().unwrap();
        if !path.starts_with("player.child") {
            continue;
        }

        let last_dot = path.rfind(".");
        if let None = last_dot {
            continue;
        }

        let path = &path[0..last_dot.unwrap()];

        let child_proto = proto_data
            .get_prototype(path)
            .expect(&format!("Expected {path} prototype!"));

        let child_id = child_proto
            .spawn(&mut commands, &proto_data, &asset_server)
            .id();

        commands.entity(player_id).add_child(child_id);
    }
}
