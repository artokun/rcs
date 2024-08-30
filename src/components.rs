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
pub struct ControlsText;

#[derive(Component)]
pub struct AttitudeText;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RCSThrusterMountPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RCSThrusterMountOrientation {
    Horizontal,
    Vertical,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RCSThrusterMountAlignment {
    Center,
    Start,
    End,
}
#[derive(Debug)]
pub struct RCSThrusterMount {
    pub position: RCSThrusterMountPosition,
    pub orientation: RCSThrusterMountOrientation,
    pub alignment: RCSThrusterMountAlignment,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct RCSThruster {
    pub active: bool,
    pub mount: RCSThrusterMount,
}
