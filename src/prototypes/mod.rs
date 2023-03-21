use bevy::prelude::{AssetServer, Commands, Res, BuildChildren, Entity};
use bevy_proto::prelude::ProtoData;
use relative_path::RelativePath;

pub mod animation;
pub mod collider;
pub mod common;
pub mod npc;
pub mod sprite;

pub fn spawn_prototype(name: &str, mut commands: &mut Commands, asset_server: &Res<AssetServer>, proto_data: &Res<ProtoData>) -> Entity {
    let proto = proto_data
        .get_prototype(name)
        .expect(&format!("Expected {} prototype!", name));
    let id = proto
        .spawn(&mut commands, &proto_data, &asset_server)
        .id();

    let proto_path = RelativePath::new("assets/prototypes");
    let paths = std::fs::read_dir(proto_path.to_path("."))
        .expect(&format!("Path {:?} not found!", proto_path.to_path(".")));

    for path in paths {
        if let Err(_) = path {
            continue;
        }

        let path = path.unwrap().file_name();
        let path = path.to_str().unwrap();
        if !path.starts_with(&format!("{}.child", name)) {
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

        commands.entity(id).add_child(child_id);
    }

    id
}