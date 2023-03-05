use std::sync::RwLock;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

#[derive(Serialize, Deserialize, Component)]
struct SpriteSheetBundleDef {
    pub texture_path: HandlePath,
    pub init_sprite: usize,
    pub tile_size: u32,
    pub sprite_width: usize,
    pub sprite_height: usize,

    #[serde(skip)]
    pub atlas_handle: RwLock<Option<Handle<TextureAtlas>>>,
}

#[typetag::serde]
impl ProtoComponent for SpriteSheetBundleDef {
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        let handle_id = self.atlas_handle.read().unwrap().as_ref().unwrap().id();
        let atlas_handle: Handle<TextureAtlas> = commands
            .get_handle(self, handle_id)
            .expect("Expected sprite sheet to be loaded!");

        commands.insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(self.init_sprite),
            texture_atlas: atlas_handle.clone(),
            ..default()
        });
    }

    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {
        let asset_server = world.resource::<AssetServer>();
        let image: Handle<Image> = asset_server.load(self.texture_path.as_str());

        let mut atlasses = world.resource_mut::<Assets<TextureAtlas>>();

        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::splat(self.tile_size as f32),
            self.sprite_width,
            self.sprite_height,
            None,
            None,
        );
        let atlas_handle = atlasses.add(atlas);

        data.insert_handle(prototype, self, atlas_handle.clone());

        *self.atlas_handle.write().unwrap() = Some(atlas_handle);
    }
}
