use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::PipelinesReady;

// Selects the level you want to load.
pub fn level_selection(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    level_data: Res<LevelData>,
    loading_state: Res<LoadingState>,
) {
    // Only trigger a load if the current level is fully loaded.
    if let LoadingState::LevelReady = loading_state.as_ref() {
        if keyboard.just_pressed(KeyCode::Digit1) {
            commands.run_system(level_data.unload_level_id);
            commands.run_system(level_data.level_1_id);
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            commands.run_system(level_data.unload_level_id);
            commands.run_system(level_data.level_2_id);
        }
    }
}

// Removes all currently loaded level assets from the game World.
pub fn unload_current_level(
    mut commands: Commands,
    mut loading_state: ResMut<LoadingState>,
    entities: Query<Entity, With<LevelComponents>>,
) {
    *loading_state = LoadingState::LevelLoading;
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
    pipelines_ready: Res<PipelinesReady>,
) {
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        // If we are still loading assets / pipelines are not fully compiled,
        // we reset the confirmation frame count.
        loading_data.confirmation_frames_count = 0;

        // Go through each asset and verify their load states.
        // Any assets that are loaded are then added to the pop list for later removal.
        let mut pop_list: Vec<usize> = Vec::new();
        for (index, asset) in loading_data.loading_assets.iter().enumerate() {
            if let Some(state) = asset_server.get_load_states(asset) {
                if let bevy::asset::RecursiveDependencyLoadState::Loaded = state.2 {
                    pop_list.push(index);
                }
            }
        }

        // Remove all loaded assets from the loading_assets list.
        for i in pop_list.iter() {
            loading_data.loading_assets.remove(*i);
        }

        // If there are no more assets being monitored, and pipelines
        // are compiled, then start counting confirmation frames.
        // Once enough confirmations have passed, everything will be
        // considered to be fully loaded.
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
            *loading_state = LoadingState::LevelReady;
        }
    }
}

pub fn load_loading_screen(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 80.0,
        ..default()
    };

    // Spawn the UI and Loading screen camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        LoadingScreen,
    ));

    // Spawn the UI that will make up the loading screen.
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            LoadingScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([TextSection::new(
                "Loading...",
                text_style.clone(),
            )]));
        });
}

pub fn display_loading_screen(
    mut loading_screen: Query<&mut Visibility, With<LoadingScreen>>,
    loading_state: Res<LoadingState>,
) {
    match loading_state.as_ref() {
        LoadingState::LevelLoading => {
            *loading_screen.get_single_mut().unwrap() = Visibility::Visible;
        }
        LoadingState::LevelReady => *loading_screen.get_single_mut().unwrap() = Visibility::Hidden,
    };
}
