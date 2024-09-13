use bevy::prelude::*;

// A `Resource` that holds the current loading state.
#[derive(Resource, Default)]
pub enum LoadingState {
    #[default]
    LevelReady,
    LevelLoading,
}

// A resource that holds the current loading data.
#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub loading_assets: Vec<UntypedHandle>,
    // Number of frames that everything needs to be ready for.
    // This is to prevent going into the fully loaded state in instances
    // where there might be a some frames between certain loading/pipelines action.
    pub confirmation_frames_target: usize,
    // Current number of confirmation frames.
    pub confirmation_frames_count: usize,
}

impl LoadingData {
    pub fn new(confirmation_frames_target: usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}
