use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource)]
pub struct LevelData {
    pub unload_level_id: SystemId,
    pub level_1_id: SystemId,
    pub level_2_id: SystemId,
}

#[derive(Component)]
pub struct LevelComponents;

#[derive(Component)]
pub struct LoadingScreen;
