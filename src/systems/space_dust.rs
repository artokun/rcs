use crate::components::{Player, SpaceDustChunk};
use crate::constants::*;
use crate::resources::ChunkManager;
use bevy::prelude::*;
use rand::Rng;

pub fn space_dust(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_transform = player_query.single();
    let current_chunk = IVec2::new(
        (player_transform.translation.x / CHUNK_SIZE).floor() as i32,
        (player_transform.translation.y / CHUNK_SIZE).floor() as i32,
    );

    // Spawn new chunks
    for dy in -1..=1 {
        for dx in -1..=1 {
            let chunk_coords = current_chunk + IVec2::new(dx, dy);
            if !chunk_manager.loaded_chunks.contains_key(&chunk_coords) {
                spawn_chunk(&mut commands, chunk_coords, &mut chunk_manager);
            }
        }
    }

    // Despawn distant chunks
    let mut to_remove = Vec::new();
    for (chunk_coords, &entity) in chunk_manager.loaded_chunks.iter() {
        if (*chunk_coords - current_chunk).abs().max_element() > 1 {
            commands.entity(entity).despawn_recursive();
            to_remove.push(*chunk_coords);
        }
    }
    for coords in to_remove {
        chunk_manager.loaded_chunks.remove(&coords);
    }
}

fn spawn_chunk(commands: &mut Commands, chunk_coords: IVec2, chunk_manager: &mut ChunkManager) {
    let mut rng = rand::thread_rng();
    let chunk_world_position = Vec2::new(
        chunk_coords.x as f32 * CHUNK_SIZE,
        chunk_coords.y as f32 * CHUNK_SIZE,
    );

    let chunk_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(chunk_world_position.extend(0.0)),
                ..default()
            },
            SpaceDustChunk { chunk_coords },
        ))
        .with_children(|parent| {
            for _ in 0..PARTICLES_PER_CHUNK {
                let x = rng.gen_range(0.0..CHUNK_SIZE);
                let y = rng.gen_range(0.0..CHUNK_SIZE);
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, rng.gen_range(0.01..0.1)),
                        custom_size: Some(Vec2::new(
                            rng.gen_range(1.0..3.0),
                            rng.gen_range(1.0..3.0),
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                });
            }
        })
        .id();

    chunk_manager
        .loaded_chunks
        .insert(chunk_coords, chunk_entity);
}
