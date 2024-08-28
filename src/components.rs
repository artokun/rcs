use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub thrust: f32,
    pub torque: f32,
}

#[derive(Component)]
pub struct SpaceDustChunk {
    #[allow(dead_code)]
    pub chunk_coords: IVec2,
}

#[derive(Component)]
pub struct FPSText;

#[derive(Component)]
pub struct AttitudeText;
