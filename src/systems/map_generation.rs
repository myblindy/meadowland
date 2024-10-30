use crate::plugins::game_world_plugin::GameState;
use crate::resources::biomes::{Biome, Biomes};
use crate::GameWorld;
use bevy::tasks::futures_lite::future;
use bevy::{prelude::*, tasks::*};
use bevy_ecs_tilemap::map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::prelude::get_tilemap_center_transform;
use bevy_ecs_tilemap::tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex};
use bevy_ecs_tilemap::TilemapBundle;
use noise::{NoiseFn, OpenSimplex};
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct MapGenerationTask(Task<Vec<MapGenerationCell>>);

struct MapGenerationWave {
    frequency: f64,
    amplitude: f64,
    noise: OpenSimplex,
}

#[derive(Default, Clone)]
struct MapGenerationCell {
    height: f64,
    biome_index: i32,
}

impl Biome {
    fn matches(&self, height: f64, moisture: f64, heat: f64) -> bool {
        height >= self.min_height && moisture >= self.min_moisture && heat >= self.min_heat
    }

    fn get_difference(&self, height: f64, moisture: f64, heat: f64) -> f64 {
        (height - self.min_height).abs()
            + (moisture - self.min_moisture).abs()
            + (heat - self.min_heat).abs()
    }
}

pub fn start_map_generation(
    mut commands: Commands,
    biomes: Res<Assets<Biomes>>,
    game_world: Res<GameWorld>,
) {
    let Some((_, biomes)) = biomes.iter().next() else {
        panic!("Biomes not loaded")
    };
    let biomes = biomes.biomes.clone();

    let (width, height) = (game_world.width(), game_world.height());

    let task_pool = AsyncComputeTaskPool::get();
    let task = task_pool.spawn(async move {
        // set up the waves
        let height_waves = vec![
            MapGenerationWave {
                frequency: 0.004,
                amplitude: 1.0,
                noise: OpenSimplex::new(rand::random()),
            },
            MapGenerationWave {
                frequency: 0.02,
                amplitude: 0.5,
                noise: OpenSimplex::new(rand::random()),
            },
        ];
        let moisture_waves = vec![MapGenerationWave {
            frequency: 0.02,
            amplitude: 1.0,
            noise: OpenSimplex::new(rand::random()),
        }];
        let heat_waves = vec![
            MapGenerationWave {
                frequency: 0.02,
                amplitude: 1.0,
                noise: OpenSimplex::new(rand::random()),
            },
            MapGenerationWave {
                frequency: 0.01,
                amplitude: 0.5,
                noise: OpenSimplex::new(rand::random()),
            },
        ];

        let mut result = vec![MapGenerationCell::default(); (width * height) as usize];
        for x in 0..width {
            for y in 0..height {
                let height_value = generate_noise_value(x, y, &height_waves);
                let moisture_value = generate_noise_value(x, y, &moisture_waves);
                let heat_value = generate_noise_value(x, y, &heat_waves);

                let mut best_biome_index = -1;
                let mut best_difference = f64::MAX;

                for biome_index in 0..biomes.len() {
                    let biome = &biomes[biome_index];
                    if biome.matches(height_value, moisture_value, heat_value) {
                        let difference =
                            biome.get_difference(height_value, moisture_value, heat_value);
                        if difference < best_difference {
                            best_biome_index = biome_index as i32;
                            best_difference = difference;
                        }
                    }
                }

                result[(y * width + x) as usize] = MapGenerationCell {
                    height: height_value,
                    biome_index: best_biome_index,
                };
            }
        }

        result
    });

    commands.insert_resource(MapGenerationTask(task));
}

fn generate_noise_value(x: u32, y: u32, waves: &[MapGenerationWave]) -> f64 {
    let mut result = 0.0;
    let mut normalization = 0.0;

    for wave in waves {
        result += wave.noise.get([
            x as f64 * wave.frequency as f64,
            y as f64 * wave.frequency as f64,
        ]) * wave.amplitude;
        normalization += wave.amplitude;
    }

    result / normalization
}

pub fn check_map_generation_finished(
    mut commands: Commands,
    mut task: ResMut<MapGenerationTask>,
    mut game_world: ResMut<GameWorld>,
    biomes: Res<Assets<Biomes>>,
    mut app_next_state: ResMut<NextState<GameState>>,
) {
    let status = block_on(future::poll_once(&mut task.0));
    if let Some(result) = status {
        commands.remove_resource::<MapGenerationTask>();
        setup_tileset(
            commands,
            &biomes.iter().next().unwrap().1.biomes,
            &result,
            &mut game_world,
        );
        app_next_state.set(GameState::Main);
    }
}

fn setup_tileset(
    mut commands: Commands,
    biomes: &Vec<Biome>,
    result: &Vec<MapGenerationCell>,
    game_world: &mut GameWorld,
) {
    let map_size = TilemapSize {
        x: game_world.width(),
        y: game_world.height(),
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..game_world.width() {
        for y in 0..game_world.height() {
            let cell = &result[(y * game_world.width() + x) as usize];
            let biome = &biomes[cell.biome_index as usize];
            let tile_pos = TilePos { x, y };
            let tile = commands.spawn((
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                },
                TileTextureIndex(
                    game_world
                        .texture_handles_for_biome(biome)
                        .choose(&mut game_world.rng())
                        .unwrap()
                        .1,
                ),
            ));
            tile_storage.set(&tile_pos, tile.id());
        }
    }

    let tile_size = TilemapTileSize {
        x: game_world.cell_size() as f32,
        y: game_world.cell_size() as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(game_world.texture_handles_for_all_biomes()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });
}
