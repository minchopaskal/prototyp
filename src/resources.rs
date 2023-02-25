use std::collections::HashMap;

use bevy::prelude::*;
use tiled::PropertyValue;

#[derive(Resource)]
pub struct UiSettings {
    pub show_debug_window: bool,
}

#[derive(Resource)]
pub struct CursorPos(pub Vec3);

impl Default for CursorPos {
    fn default() -> Self {
        CursorPos(Vec3::new(-1000.0, -1000.0, 0.0))
    }
}

#[derive(Resource, Default)]
pub struct TilesProperties {
    // Map TilesetId -> (Map TileId -> (Map PropName -> PropValue))
    pub props: Vec<Vec<HashMap<String, PropertyValue>>>,
}

#[derive(Resource, Default)]
pub struct VelocityMultiplier(pub f32);

#[derive(Debug)]
pub struct SignData {
    pub x: f32,
    pub y: f32,
    pub id: u32,
}

#[derive(Resource, Default)]
pub struct SignsPool {
    pub signs: Vec<SignData>,
}
