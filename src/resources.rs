use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashMap<IVec2, Entity>,
}
