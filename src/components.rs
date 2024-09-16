use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource)]
pub struct LevelData {
    #[allow(dead_code)]
    pub unload_level_id: SystemId,
    #[allow(dead_code)]
    pub level_1_id: SystemId,
    #[allow(dead_code)]
    pub level_2_id: SystemId,
}

#[derive(Component)]
pub struct LevelComponents;

#[derive(Component)]
pub struct LoadingScreen;
